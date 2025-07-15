pub enum TaskEndpoint {
    Image,
    ImageBinary,
    User,
    UserSubscription,
    Transactions,
    Deposits,
    Config,
    Statistics,
    NodeStatistics,
    ProverNodeSummary,
    OnlineNodesSummary,
    Tasks,
    ConciseTasks,
    TaskExternalHostTable,
    Round1Batch,
    Round2Batch,
    FinalBatch,
    Logs,
    ArchiveSummary,
    ArchiveTaskVolumeList,
    ArchiveAutoSubmitTaskVolumeList,
    ArchiveTask(String),
    ArchiveAutoSubmitNetworks(String),
    ArchiveAutoSubmitInfoByTask(String, u32),
    ArchiveAutoSubmitInfo(String, u32),
    ArchiveConfig,
    ArchiveTaskVolume(String),
    ArchiveAutoSubmitVolume(String),
    ArchiveArchiveQuery,
    Pay,
    Subscribe,
    Setup,
    Prove,
    Deploy,
    Reset,
    Modify,
    SetMaintenanceMode,
    ForceUnprovableToReprocess,
    ForceDryrunFailsToReprocess,
    EstimatedProofFee,
    ProverNodeTimerangeStats,
}

impl TaskEndpoint {
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
            _ => "".to_string(),
        }
    }
}
