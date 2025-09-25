use serde::Deserialize;
use serde::Serialize;

use super::util::IntoMultipartForm;
use super::util::SerializationAttributes;

/// Represents all available API endpoint paths for the `ZkWasm` service.
///
/// Each variant corresponds to a specific endpoint and provides a way to build parameterized request paths.
pub enum TaskEndpoint {
    /// `/image`
    Image,
    /// `/imagebinary`
    ImageBinary,
    /// `/user`
    User,
    /// `/user_subscription`
    UserSubscription,
    /// `/transactions`
    Transactions,
    /// `/deposits`
    Deposits,
    /// `/config`
    Config,
    /// `/statistics`
    Statistics,
    /// `/node_statistics`
    NodeStatistics,
    /// `/prover_node_summary`
    ProverNodeSummary,
    /// `/online_nodes_summary`
    OnlineNodesSummary,
    /// `/tasks`
    Tasks,
    /// `/tasklist`
    ConciseTasks,
    /// `/task_external_host_table`
    TaskExternalHostTable,
    /// `/round1_batch_proofs`
    Round1Batch,
    /// `/round2_batch_proofs`
    Round2Batch,
    /// `/final_batch_proofs`
    FinalBatch,
    /// `/logs`
    Logs,
    /// `/archive/summary`
    ArchiveSummary,
    /// `/archive/task_volume_list`
    ArchiveTaskVolumeList,
    /// `/archive/auto_submit_volume_list`
    ArchiveAutoSubmitTaskVolumeList,
    /// `/archive/task`
    ArchiveTask(String),
    /// `/archive/auto_submit_networks`
    ArchiveAutoSubmitNetworks(String),
    /// `/archive/auto_submit_info_by_task`
    ArchiveAutoSubmitInfoByTask(String, u32),
    /// `/archive/auto_submit_info`
    ArchiveAutoSubmitInfo(String, u32),
    /// `/archive/config`
    ArchiveConfig,
    /// `/archive/task_volume`
    ArchiveTaskVolume(String),
    /// `/archive/auto_submit_volume`
    ArchiveAutoSubmitVolume(String),
    /// `/archive/archive_query`
    ArchiveArchiveQuery,
    /// `/pay`
    Pay,
    /// `/subscribe`
    Subscribe,
    /// `/setup`
    Setup,
    /// `/prove`
    Prove,
    /// `/deploy`
    Deploy,
    /// `/reset`
    Reset,
    /// `/modify`
    Modify,
    /// `/set_maintenance_mode`
    SetMaintenanceMode,
    /// `/force_unprovable_to_reprocess`
    ForceUnprovableToReprocess,
    /// `/force_dryrun_fails_to_reprocess`
    ForceDryrunFailsToReprocess,
    /// `/estimated_proof_fee`
    EstimatedProofFee,
    /// `/prover_node_timerange_stats`
    ProverNodeTimerangeStats,
}

impl TaskEndpoint {
    #[must_use]
    pub fn as_path(&self) -> &'static str {
        match self {
            TaskEndpoint::Image => "image",
            TaskEndpoint::ImageBinary => "imagebinary",
            TaskEndpoint::User => "user",
            TaskEndpoint::UserSubscription => "user_subscription",
            TaskEndpoint::Transactions => "transactions",
            TaskEndpoint::Deposits => "deposits",
            TaskEndpoint::Config => "config",
            TaskEndpoint::Statistics => "statistics",
            TaskEndpoint::NodeStatistics => "node_statistics",
            TaskEndpoint::ProverNodeSummary => "prover_node_summary",
            TaskEndpoint::OnlineNodesSummary => "online_nodes_summary",
            TaskEndpoint::Tasks => "tasks",
            TaskEndpoint::ConciseTasks => "tasklist",
            TaskEndpoint::TaskExternalHostTable => "task_external_host_table",
            TaskEndpoint::Round1Batch => "round1_batch_proofs",
            TaskEndpoint::Round2Batch => "round2_batch_proofs",
            TaskEndpoint::FinalBatch => "final_batch_proofs",
            TaskEndpoint::Logs => "logs",
            TaskEndpoint::ArchiveSummary => "archive/summary",
            TaskEndpoint::ArchiveTaskVolumeList => "archive/task_volume_list",
            TaskEndpoint::ArchiveAutoSubmitTaskVolumeList => "archive/auto_submit_volume_list",
            TaskEndpoint::ArchiveTask(_) => "archive/task",
            TaskEndpoint::ArchiveAutoSubmitNetworks(_) => "archive/auto_submit_networks",
            TaskEndpoint::ArchiveAutoSubmitInfoByTask(..) => "archive/auto_submit_info_by_task",
            TaskEndpoint::ArchiveAutoSubmitInfo(..) => "archive/auto_submit_info",
            TaskEndpoint::ArchiveConfig => "archive/config",
            TaskEndpoint::ArchiveTaskVolume(_) => "archive/task_volume",
            TaskEndpoint::ArchiveAutoSubmitVolume(_) => "archive/auto_submit_volume",
            TaskEndpoint::ArchiveArchiveQuery => "archive/archive_query",
            TaskEndpoint::Pay => "pay",
            TaskEndpoint::Subscribe => "subscribe",
            TaskEndpoint::Setup => "setup",
            TaskEndpoint::Prove => "prove",
            TaskEndpoint::Deploy => "deploy",
            TaskEndpoint::Reset => "reset",
            TaskEndpoint::Modify => "modify",
            TaskEndpoint::SetMaintenanceMode => "admin/set_maintenance_mode",
            TaskEndpoint::ForceUnprovableToReprocess => "admin/force_unprovable_to_reprocess",
            TaskEndpoint::ForceDryrunFailsToReprocess => "admin/force_dryrun_fails_to_reprocess",
            TaskEndpoint::EstimatedProofFee => "estimated_proof_fee",
            TaskEndpoint::ProverNodeTimerangeStats => "prover_node_timerange_stats",
        }
    }

    #[must_use]
    pub fn path_params(&self) -> String {
        match self {
            TaskEndpoint::ArchiveTask(fst)
            | TaskEndpoint::ArchiveAutoSubmitNetworks(fst)
            | TaskEndpoint::ArchiveTaskVolume(fst)
            | TaskEndpoint::ArchiveAutoSubmitVolume(fst) => {
                format!("/{fst}")
            }
            TaskEndpoint::ArchiveAutoSubmitInfoByTask(fst, snd) | TaskEndpoint::ArchiveAutoSubmitInfo(fst, snd) => {
                format!("/{fst}/{snd}")
            }
            _ => String::new(),
        }
    }
}

/// A standardized result wrapper for responses from requests.
///
/// This struct ensures a consistent response format by always returning a `success` flag alongside the actual
/// `result` payload.
#[derive(Deserialize, Serialize)]
pub struct RequestResult<T: Serialize> {
    pub success: bool,
    pub result: T,
}

/// A client for interacting with `ZkWasm` service endpoints.
///
/// This struct holds the base URL of the service and provides implementations for common HTTP operations such as `GET`
/// and `POST`.
pub struct ZkWasmServiceEndpoint {
    endpoint: String,
}

impl ZkWasmServiceEndpoint {
    #[must_use]
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    fn to_path(&self, path: &TaskEndpoint) -> String {
        format!("{}/{}{}", self.endpoint, path.as_path(), path.path_params())
    }

    async fn execute<V: for<'de> Deserialize<'de> + Serialize>(
        mut req: reqwest::RequestBuilder,
        signature: Option<String>,
    ) -> anyhow::Result<V> {
        if let Some(sig) = signature {
            req = req.header("x-eth-signature", sig);
        }

        Ok(req
            .send()
            .await?
            .text()
            .await
            .map(|resp| {
                serde_json::from_str::<RequestResult<V>>(&resp).inspect_err(|e| {
                    println!("Error: {e}");
                    println!("Response: {resp}");
                })
            })??
            .result)
    }

    /// Sends a GET request to the given [`TaskEndpoint`] with optional parameters and signature.
    ///
    /// # Type Parameters
    ///
    /// - `U`: The type of the request parameters. Must implement [`Serialize`].
    /// - `V`: The expected response type. Must implement both [`Deserialize`] and [`Serialize`].
    ///
    /// # Arguments
    ///
    /// - `path`: The [`TaskEndpoint`] to which the request will be sent.
    /// - `params`: The query parameters to include in the request.
    /// - `signature`: An optional request signature. If `Some`, the request will be signed;
    ///   if `None`, it will be sent unsigned.
    ///
    /// # Returns
    ///
    /// Returns [`anyhow::Result`] containing the deserialized response of type `V` on success,
    /// or an error if the request fails or the response cannot be deserialized.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The request cannot be constructed or sent.
    /// - The server responds with an error status code.
    /// - The response body cannot be deserialized into type `V`.
    pub async fn get<U: Serialize, V: for<'de> Deserialize<'de> + Serialize>(
        &self,
        path: TaskEndpoint,
        params: U,
        signature: Option<String>,
    ) -> anyhow::Result<V> {
        let base = self.to_path(&path);
        let encoded = serde_urlencoded::to_string(params)?;
        let url = format!("{}{}{}", base, if encoded.is_empty() { "" } else { "?" }, encoded);
        println!("GET {url}");

        let req = reqwest::Client::new().get(url);
        Self::execute(req, signature).await
    }

    /// Sends a POST request to the given [`TaskEndpoint`] with parameters and an optional signature.
    ///
    /// # Type Parameters
    ///
    /// - `U`: The type of the request body. Must implement [`Serialize`] and [`SerializationAttributes`].
    /// - `V`: The expected response type. Must implement both [`Deserialize`] and [`Serialize`].
    ///
    /// # Arguments
    ///
    /// - `path`: The [`TaskEndpoint`] to which the request will be sent.
    /// - `body`: The body of the request, which must be serializable and may include additional
    ///   serialization attributes.
    /// - `signature`: An optional request signature. If `Some`, the request will be signed;
    ///   if `None`, it will be sent unsigned.
    ///
    /// # Returns
    ///
    /// Returns [`anyhow::Result`] containing the deserialized response of type `V` on success,
    /// or an error if the request fails or the response cannot be deserialized.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The request cannot be constructed or sent.
    /// - The server responds with an error status code.
    /// - The request body cannot be serialized according to `U`.
    /// - The response body cannot be deserialized into type `V`.
    pub async fn post<U: Serialize + SerializationAttributes, V: for<'de> Deserialize<'de> + Serialize>(
        &self,
        path: TaskEndpoint,
        body: U,
        signature: Option<String>,
    ) -> anyhow::Result<V> {
        let url = self.to_path(&path);
        println!("POST {url}");

        let post = reqwest::Client::new().post(url);
        let req = if U::requires_json_body() {
            post.json(&body)
        } else {
            post.multipart(IntoMultipartForm::into_multipart_form(body)?)
        };

        Self::execute(req, signature).await
    }
}
