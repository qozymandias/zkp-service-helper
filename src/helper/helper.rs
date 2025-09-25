use super::endpoint::TaskEndpoint;
use super::endpoint::ZkWasmServiceEndpoint;
use super::util::sign_object;
use crate::interface::AddImageParams;
use crate::interface::AddProveTaskRestrictions;
use crate::interface::AddTaskResult;
use crate::interface::AdminRequestType;
use crate::interface::AppConfig;
use crate::interface::ArchiveMetadataOverview;
use crate::interface::ArchiveQuery;
use crate::interface::ArchiveServerConfig;
use crate::interface::ArchiveVolumeMetadata;
use crate::interface::ArchivedFinalBatchProof;
use crate::interface::ArchivedFinalProofNetworkInfo;
use crate::interface::AutoSubmitProof;
use crate::interface::AutoSubmitProofQuery;
use crate::interface::AutoSubmitProofStatus;
use crate::interface::BaseAddImageParams;
use crate::interface::BaseProvingParams;
use crate::interface::BaseResetImageParams;
use crate::interface::ConciseTask;
use crate::interface::CustomContext;
use crate::interface::DeployParams;
use crate::interface::ERC20DepositInfo;
use crate::interface::EmptyParams;
use crate::interface::EstimatedProofFee;
use crate::interface::EstimatedProofFeeParams;
use crate::interface::ForceDryrunFailsToReprocessParams;
use crate::interface::ForceUnprovableToReprocessParams;
use crate::interface::Image;
use crate::interface::InitialContext;
use crate::interface::LogQuery;
use crate::interface::MaintenanceModeType;
use crate::interface::ModifyImageParams;
use crate::interface::NodeStatisticsQueryParams;
use crate::interface::ObjectId;
use crate::interface::OnlineNodesSummary;
use crate::interface::PaginatedQuery;
use crate::interface::PaginationParams;
use crate::interface::PaginationResult;
use crate::interface::PaymentParams;
use crate::interface::ProofSubmitMode;
use crate::interface::ProvePaymentSrc;
use crate::interface::ProverNode;
use crate::interface::ProverNodeTimeRangeStats;
use crate::interface::ProverNodeTimeRangeStatsParams;
use crate::interface::ProverNodesSummary;
use crate::interface::ProvingParams;
use crate::interface::QueryImageParams;
use crate::interface::QueryParams;
use crate::interface::ResetContext;
use crate::interface::ResetImageParams;
use crate::interface::Round1Info;
use crate::interface::Round1InfoQuery;
use crate::interface::Round1Status;
use crate::interface::Round2Info;
use crate::interface::Round2InfoQuery;
use crate::interface::Round2Status;
use crate::interface::SetMaintenanceModeParams;
use crate::interface::StatisticsInfo;
use crate::interface::Subscription;
use crate::interface::SubscriptionDuration;
use crate::interface::SubscriptionRequest;
use crate::interface::SubscriptionType;
use crate::interface::Task;
use crate::interface::TaskExternalHostTable;
use crate::interface::TaskExternalHostTableParams;
use crate::interface::TaskStatus;
use crate::interface::TaskType;
use crate::interface::TransactionInfo;
use crate::interface::TxHistoryQueryParams;
use crate::interface::User;
use crate::interface::UserQueryParams;
use crate::interface::VolumeDetailQuery;
use crate::interface::VolumeDetailResponse;
use crate::interface::VolumeListQuery;

/// A helper struct for interacting with the `ZkWasm` service endpoint.
///
/// This struct encapsulates a [`ZkWasmServiceEndpoint`] and provides convenience functions for interacting with the API
/// endpoints.
pub struct ZkWasmServiceHelper {
    endpoint: ZkWasmServiceEndpoint,
}

impl ZkWasmServiceHelper {
    #[must_use]
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint: ZkWasmServiceEndpoint::new(endpoint),
        }
    }

    pub async fn query_image(&self, md5: String) -> anyhow::Result<Option<Image>> {
        self.endpoint
            .get::<_, Vec<Option<Image>>>(TaskEndpoint::Image, QueryImageParams { md5 }, None)
            .await
            .map(|mut res| res.remove(0))
    }

    pub async fn query_image_binary(&self, md5: String) -> anyhow::Result<Vec<u8>> {
        self.endpoint
            .get(TaskEndpoint::ImageBinary, QueryImageParams { md5 }, None)
            .await
    }

    pub async fn query_user(&self, user_address: String) -> anyhow::Result<Option<User>> {
        self.endpoint
            .get(TaskEndpoint::User, UserQueryParams { user_address }, None)
            .await
    }

    pub async fn query_user_subscription(&self, user_address: String) -> anyhow::Result<Option<Subscription>> {
        self.endpoint
            .get(TaskEndpoint::UserSubscription, UserQueryParams { user_address }, None)
            .await
    }

    pub async fn query_tx_history(
        &self,
        user_address: String,
    ) -> anyhow::Result<PaginationResult<Vec<TransactionInfo>>> {
        self.endpoint
            .get(
                TaskEndpoint::Transactions,
                TxHistoryQueryParams {
                    user_address,
                    start: None,
                    total: None,
                },
                None,
            )
            .await
    }

    pub async fn query_deposit_history(
        &self,
        user_address: String,
    ) -> anyhow::Result<PaginationResult<Vec<ERC20DepositInfo>>> {
        self.endpoint
            .get(
                TaskEndpoint::Deposits,
                TxHistoryQueryParams {
                    user_address,
                    start: None,
                    total: None,
                },
                None,
            )
            .await
    }

    pub async fn query_config(&self) -> anyhow::Result<AppConfig> {
        self.endpoint.get(TaskEndpoint::Config, EmptyParams {}, None).await
    }

    pub async fn query_statistics(&self) -> anyhow::Result<StatisticsInfo> {
        self.endpoint.get(TaskEndpoint::Statistics, EmptyParams {}, None).await
    }

    pub async fn query_node_statistics(
        &self,
        address: Option<String>,
        start: Option<u64>,
        total: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<ProverNode>>> {
        self.endpoint
            .get(
                TaskEndpoint::NodeStatistics,
                NodeStatisticsQueryParams {
                    address,
                    start,
                    total,
                },
                None,
            )
            .await
    }

    pub async fn query_prover_node_summary(&self) -> anyhow::Result<ProverNodesSummary> {
        self.endpoint.get(TaskEndpoint::ProverNodeSummary, EmptyParams {}, None).await
    }

    pub async fn query_online_node_summary(&self) -> anyhow::Result<OnlineNodesSummary> {
        self.endpoint.get(TaskEndpoint::OnlineNodesSummary, EmptyParams {}, None).await
    }

    pub async fn query_logs(&self, id: String, user_address: String, private_key: String) -> anyhow::Result<String> {
        let params = LogQuery { id, user_address };
        let signature = sign_object(&params, private_key).await?;
        self.endpoint.get(TaskEndpoint::Logs, params, Some(signature)).await
    }

    pub async fn query_estimated_proof_fee(
        &self,
        user_address: String,
        md5: String,
        proof_submit_mode: ProofSubmitMode,
    ) -> anyhow::Result<EstimatedProofFee> {
        self.endpoint
            .get(
                TaskEndpoint::EstimatedProofFee,
                EstimatedProofFeeParams {
                    user_address,
                    md5,
                    proof_submit_mode,
                },
                None,
            )
            .await
    }

    pub async fn query_prover_node_timerange_stats(
        &self,
        query: ProverNodeTimeRangeStatsParams,
    ) -> anyhow::Result<Vec<ProverNodeTimeRangeStats>> {
        self.endpoint.post(TaskEndpoint::ProverNodeTimerangeStats, query, None).await
    }

    pub async fn query_tasks(
        &self,
        user_address: Option<String>,
        md5: Option<String>,
        id: Option<String>,
        tasktype: Option<TaskType>,
        taskstatus: Option<TaskStatus>,
        start: Option<u64>,
        total: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<Task>>> {
        self.endpoint
            .get(
                TaskEndpoint::Tasks,
                QueryParams {
                    user_address,
                    md5,
                    id,
                    tasktype,
                    taskstatus,
                    start,
                    total,
                },
                None,
            )
            .await
    }

    pub async fn query_tasks_from_ids(&self, ids: Vec<String>) -> anyhow::Result<Vec<Task>> {
        const QUERY_TASKS_FROM_IDS_MAX_SIZE_INPUT: usize = 10;

        if ids.len() > QUERY_TASKS_FROM_IDS_MAX_SIZE_INPUT {
            return Err(anyhow::anyhow!(
                "Cannot be larger than max {QUERY_TASKS_FROM_IDS_MAX_SIZE_INPUT}"
            ));
        }
        let mut out = vec![];
        for id in ids {
            let mut tasks = self.query_tasks(None, None, Some(id), None, None, None, Some(1)).await?.data;
            if tasks.is_empty() {
                continue;
            }
            out.push(tasks.remove(0));
        }
        Ok(out)
    }

    pub async fn query_task_from_id(&self, id: String) -> anyhow::Result<Option<Task>> {
        let mut tasks = self.query_tasks_from_ids(vec![id]).await?;
        if tasks.is_empty() {
            Ok(None)
        } else {
            Ok(Some(tasks.remove(0)))
        }
    }

    pub async fn query_concise_tasks(
        &self,
        user_address: Option<String>,
        md5: Option<String>,
        id: Option<String>,
        tasktype: Option<TaskType>,
        taskstatus: Option<TaskStatus>,
        start: Option<u64>,
        total: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<ConciseTask>>> {
        self.endpoint
            .get(
                TaskEndpoint::ConciseTasks,
                QueryParams {
                    user_address,
                    md5,
                    id,
                    tasktype,
                    taskstatus,
                    start,
                    total,
                },
                None,
            )
            .await
    }

    pub async fn get_task_external_host_table(&self, id: String) -> anyhow::Result<TaskExternalHostTable> {
        self.endpoint
            .get(TaskEndpoint::TaskExternalHostTable, TaskExternalHostTableParams { id }, None)
            .await
    }

    pub async fn query_auto_submit_proofs(
        &self,
        id: Option<String>,
        task_id: Option<String>,
        status: Option<AutoSubmitProofStatus>,
        circuit_size: Option<u32>,
        chain_id: Option<u32>,
        start: Option<u64>,
        total: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<AutoSubmitProof>>> {
        self.endpoint
            .get(
                TaskEndpoint::Round1Batch,
                PaginatedQuery {
                    query: AutoSubmitProofQuery {
                        id,
                        task_id,
                        status,
                        circuit_size,
                        chain_id,
                    },
                    pagination: PaginationParams { total, start },
                },
                None,
            )
            .await
    }

    pub async fn query_round1_info(
        &self,
        id: Option<String>,
        auto_submit_queue_id: Option<String>,
        task_id: Option<String>,
        status: Option<Round1Status>,
        circuit_size: Option<u32>,
        chain_id: Option<u32>,
        start: Option<u64>,
        total: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<Round1Info>>> {
        self.endpoint
            .get(
                TaskEndpoint::Round2Batch,
                PaginatedQuery {
                    query: Round1InfoQuery {
                        id,
                        // Note: `round_1_id` should never be used because it's never exposed to the user.
                        round_1_id: auto_submit_queue_id,
                        task_id,
                        status,
                        circuit_size,
                        chain_id,
                    },
                    pagination: PaginationParams { total, start },
                },
                None,
            )
            .await
    }

    pub async fn query_round2_info(
        &self,
        id: Option<String>,
        round_2_id: Option<String>,
        task_id: Option<String>,
        status: Option<Round2Status>,
        chain_id: Option<u32>,
        start: Option<u64>,
        total: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<Round2Info>>> {
        self.endpoint
            .get(
                TaskEndpoint::FinalBatch,
                PaginatedQuery {
                    query: Round2InfoQuery {
                        id,
                        round_2_id,
                        task_id,
                        status,
                        chain_id,
                    },
                    pagination: PaginationParams { total, start },
                },
                None,
            )
            .await
    }

    pub async fn query_archive_summary(&self) -> anyhow::Result<ArchiveMetadataOverview> {
        self.endpoint.get(TaskEndpoint::ArchiveSummary, EmptyParams {}, None).await
    }

    pub async fn query_archive_task_volume_list(
        &self,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<ArchiveVolumeMetadata>>> {
        self.endpoint
            .get(TaskEndpoint::ArchiveTaskVolumeList, VolumeListQuery { start, limit }, None)
            .await
    }

    pub async fn query_archive_auto_submit_task_volume_list(
        &self,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<ArchiveVolumeMetadata>>> {
        self.endpoint
            .get(
                TaskEndpoint::ArchiveAutoSubmitTaskVolumeList,
                VolumeListQuery { start, limit },
                None,
            )
            .await
    }

    pub async fn query_archived_task(&self, task_id: String) -> anyhow::Result<Task> {
        self.endpoint
            .get(TaskEndpoint::ArchiveTask(task_id), EmptyParams {}, None)
            .await
    }

    pub async fn query_archived_auto_submit_networks_by_task_id(
        &self,
        task_id: String,
    ) -> anyhow::Result<Vec<ArchivedFinalProofNetworkInfo>> {
        self.endpoint
            .get(TaskEndpoint::ArchiveAutoSubmitNetworks(task_id), EmptyParams {}, None)
            .await
    }

    pub async fn query_archived_auto_submit_info_by_task_id(
        &self,
        task_id: String,
        chain_id: u32,
    ) -> anyhow::Result<ArchivedFinalBatchProof> {
        self.endpoint
            .get(
                TaskEndpoint::ArchiveAutoSubmitInfoByTask(task_id, chain_id),
                EmptyParams {},
                None,
            )
            .await
    }

    pub async fn query_archived_auto_submit_info_by_archive_id(
        &self,
        id: String,
        chain_id: u32,
    ) -> anyhow::Result<ArchivedFinalBatchProof> {
        self.endpoint
            .get(TaskEndpoint::ArchiveAutoSubmitInfo(id, chain_id), EmptyParams {}, None)
            .await
    }

    pub async fn query_archive_server_config(&self) -> anyhow::Result<ArchiveServerConfig> {
        self.endpoint.get(TaskEndpoint::ArchiveConfig, EmptyParams {}, None).await
    }

    pub async fn query_archive_task_volume(
        &self,
        volume_name: String,
        tasks_start: Option<u64>,
        tasks_limit: Option<u64>,
    ) -> anyhow::Result<VolumeDetailResponse> {
        self.endpoint
            .get(
                TaskEndpoint::ArchiveTaskVolume(volume_name),
                VolumeDetailQuery {
                    tasks_start,
                    tasks_limit,
                },
                None,
            )
            .await
    }

    pub async fn query_archive_auto_submit_volume(
        &self,
        volume_name: String,
        tasks_start: Option<u64>,
        tasks_limit: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<ArchivedFinalBatchProof>>> {
        self.endpoint
            .get(
                TaskEndpoint::ArchiveAutoSubmitVolume(volume_name),
                VolumeDetailQuery {
                    tasks_start,
                    tasks_limit,
                },
                None,
            )
            .await
    }

    pub async fn query_archive(
        &self,
        task_id: Option<String>,
        md5: Option<String>,
        start_timestamp: Option<String>,
        end_timestamp: Option<String>,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> anyhow::Result<PaginationResult<Vec<ConciseTask>>> {
        self.endpoint
            .get(
                TaskEndpoint::ArchiveArchiveQuery,
                ArchiveQuery {
                    task_id,
                    md5,
                    start_timestamp,
                    end_timestamp,
                    start,
                    limit,
                },
                None,
            )
            .await
    }

    pub async fn add_payment(&self, txhash: String) -> anyhow::Result<ERC20DepositInfo> {
        self.endpoint.post(TaskEndpoint::Pay, PaymentParams { txhash }, None).await
    }

    pub async fn add_subscription(
        &self,
        subscriber_address: String,
        subscription_type: SubscriptionType,
        duration: SubscriptionDuration,
        payment_hash: String,
    ) -> anyhow::Result<ERC20DepositInfo> {
        self.endpoint
            .post(
                TaskEndpoint::Subscribe,
                SubscriptionRequest {
                    subscriber_address,
                    subscription_type,
                    duration,
                    payment_hash,
                },
                None,
            )
            .await
    }

    pub async fn setup_image(
        &self,
        name: String,
        image: Vec<u8>,
        image_md5: String,
        user_address: String,
        description_url: String,
        avator_url: String,
        circuit_size: u32,
        prove_payment_src: ProvePaymentSrc,
        auto_submit_network_ids: Vec<u32>,
        add_prove_task_restrictions: Option<AddProveTaskRestrictions>,
        inherited_merkle_data_md5: Option<String>,
        context: InitialContext,
        private_key: String,
    ) -> anyhow::Result<AddTaskResult> {
        let params = AddImageParams {
            base: BaseAddImageParams {
                name,
                image_md5,
                image,
                user_address,
                description_url,
                avator_url,
                circuit_size,
                prove_payment_src,
                auto_submit_network_ids,
                add_prove_task_restrictions,
                inherited_merkle_data_md5,
            },
            context,
        };
        let signature = sign_object(&params, private_key).await?;
        self.endpoint.post(TaskEndpoint::Setup, params, Some(signature)).await
    }

    pub async fn add_prove(
        &self,
        user_address: String,
        md5: String,
        public_inputs: Vec<String>,
        private_inputs: Vec<String>,
        proof_submit_mode: ProofSubmitMode,
        context: CustomContext,
        private_key: String,
    ) -> anyhow::Result<AddTaskResult> {
        let params = ProvingParams {
            base: BaseProvingParams {
                user_address,
                md5,
                public_inputs,
                private_inputs,
                proof_submit_mode,
            },
            context,
        };
        let signature = sign_object(&params, private_key).await?;
        self.endpoint.post(TaskEndpoint::Prove, params, Some(signature)).await
    }

    #[deprecated]
    pub async fn add_deploy(
        &self,
        user_address: String,
        md5: String,
        chain_id: u32,
        private_key: String,
    ) -> anyhow::Result<()> {
        let params = DeployParams {
            user_address,
            md5,
            chain_id,
        };
        let signature = sign_object(&params, private_key).await?;
        self.endpoint.post(TaskEndpoint::Deploy, params, Some(signature)).await
    }

    pub async fn add_reset(
        &self,
        md5: String,
        circuit_size: u32,
        user_address: String,
        prove_payment_src: ProvePaymentSrc,
        auto_submit_network_ids: Vec<u32>,
        add_prove_task_restrictions: Option<AddProveTaskRestrictions>,
        context: ResetContext,
        private_key: String,
    ) -> anyhow::Result<AddTaskResult> {
        let params = ResetImageParams {
            base: BaseResetImageParams {
                md5,
                circuit_size,
                user_address,
                prove_payment_src,
                auto_submit_network_ids,
                add_prove_task_restrictions,
            },
            context,
        };
        let signature = sign_object(&params, private_key).await?;
        self.endpoint.post(TaskEndpoint::Reset, params, Some(signature)).await
    }

    pub async fn modify_image(
        &self,
        md5: String,
        user_address: String,
        description_url: String,
        avator_url: String,
        private_key: String,
    ) -> anyhow::Result<String> {
        let params = ModifyImageParams {
            md5,
            user_address,
            description_url,
            avator_url,
        };
        let signature = sign_object(&params, private_key).await?;
        self.endpoint.post(TaskEndpoint::Modify, params, Some(signature)).await
    }

    #[deprecated]
    pub async fn set_maintenance_mode(
        &self,
        mode: MaintenanceModeType,
        nonce: u64,
        request_type: AdminRequestType,
        user_address: String,
        private_key: String,
    ) -> anyhow::Result<String> {
        let params = SetMaintenanceModeParams {
            mode,
            nonce,
            request_type,
            user_address,
        };
        let signature = sign_object(&params, private_key).await?;
        self.endpoint
            .post(TaskEndpoint::SetMaintenanceMode, params, Some(signature))
            .await
    }

    pub async fn force_unprovable_to_reprocess(
        &self,
        task_ids: Vec<String>,
        user_address: String,
        private_key: String,
    ) -> anyhow::Result<Vec<ObjectId>> {
        let nonce = 0;
        let request_type = AdminRequestType::ForceTaskToReprocess;
        let params = ForceUnprovableToReprocessParams {
            task_ids,
            nonce,
            request_type,
            user_address,
        };
        let signature = sign_object(&params, private_key).await?;
        self.endpoint
            .post(TaskEndpoint::ForceUnprovableToReprocess, params, Some(signature))
            .await
    }

    pub async fn force_dryrun_fails_to_reprocess(
        &self,
        task_ids: Vec<String>,
        user_address: String,
        private_key: String,
    ) -> anyhow::Result<Vec<ObjectId>> {
        let nonce = 0;
        let request_type = AdminRequestType::ForceTaskToReprocess;
        let params = ForceDryrunFailsToReprocessParams {
            task_ids,
            nonce,
            request_type,
            user_address,
        };
        let signature = sign_object(&params, private_key).await?;
        self.endpoint
            .post(TaskEndpoint::ForceDryrunFailsToReprocess, params, Some(signature))
            .await
    }
}
