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
