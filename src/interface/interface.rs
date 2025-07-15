#![allow(dead_code, non_snake_case)]
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct EmptyParams;

#[derive(Deserialize, Serialize)]
pub struct PaginationResult<T: Serialize> {
    pub data: T,
    pub total: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ObjectId {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Deserialize, Serialize)]
pub struct QueryImageParams {
    pub md5: String,
}

#[derive(Deserialize, Serialize)]
pub struct StatisticsInfo {
    pub total_images: u64,
    pub total_proofs: u64,
    pub total_tasks: u64,
    pub total_deployed: u64,
}

#[derive(Deserialize, Serialize)]
pub struct TimingStatistics {
    pub latest_time_taken_secs: u64,
    pub latest_timestamp: String,
    pub latest_task_id: ObjectId,
}

#[derive(Deserialize, Serialize)]
pub struct NodeStatistics {
    pub successful_tasks: u64,
    pub failed_tasks: u64,
    pub total_tasks: u64,
    pub timed_out_count: u64,
    pub last_timed_out: String,
    pub last_timed_out_task_id: Option<ObjectId>,
    pub last_failed_ts: String,
    pub last_failed_task_id: Option<ObjectId>,
    pub last_failed_task_log: Option<String>,
    pub setup_timing_stats: Option<TimingStatistics>,
    pub proof_timing_stats: Option<TimingStatistics>,
}

#[derive(Deserialize, Serialize)]
pub struct VersionInfo {
    pub version: String,
}

#[derive(Deserialize, Serialize)]
pub enum ProverLevel {
    Inactive,
    Intern,
    Active,
    Certified,
}

#[derive(Deserialize, Serialize)]
pub struct LastAttemptedTask {
    pub task_id: ObjectId,
    pub timestamp: String,
}

#[derive(Deserialize, Serialize)]
pub struct OnlineNodeInfo {
    pub address: String,
    pub prover_level: ProverLevel,
    pub last_completed_dry_run_task_id: Option<ObjectId>,
    pub last_active_time: String,
    pub online: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ProverNode {
    pub address: String,
    pub statistics: NodeStatistics,
    pub version_info: Option<VersionInfo>,
    pub performance_track: String,
    pub prover_level: ProverLevel,
    pub last_attempted_task: Option<LastAttemptedTask>,
    pub online_activity: Option<OnlineNodeInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct OnlineNodesSummary {
    pub certified: Vec<OnlineNodeInfo>,
    pub active: Vec<OnlineNodeInfo>,
    pub intern: Vec<OnlineNodeInfo>,
    pub inactive: Vec<OnlineNodeInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct ProverNodesSummary {
    pub certified_prover_count: u64,
    pub active_prover_count: u64,
    pub intern_prover_count: u64,
    pub inactive_prover_count: u64,
}

#[derive(Deserialize, Serialize)]
pub struct NodeStatisticsQueryParams {
    pub address: Option<String>,
    pub start: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub enum InputContextType {
    Custom,
    ImageInitial,
    ImageCurrent,
}

#[derive(Deserialize, Serialize)]
pub enum CompressionType {
    None,
    GZip,
}

pub type ContextHexString = String;

#[derive(Deserialize, Serialize)]
pub enum TaskStatus {
    Pending,
    Processing,
    DryRunSuccess,
    DryRunFailed,
    Done,
    Fail,
    Unprovable,
    Stale,
}

#[derive(Deserialize, Serialize)]
pub struct VerifierContracts {
    pub chain_id: u32,
    pub aggregator_verifier: String,
    pub batch_verifier: String,
    pub circuit_size: u32,
}

#[derive(Deserialize, Serialize)]
pub struct TaskVerificationData {
    pub static_file_checksum: Vec<u8>,
    pub verifier_contracts: Vec<VerifierContracts>,
}

#[derive(Deserialize, Serialize)]
pub enum ProofSubmitMode {
    Manual,
    Auto,
}

#[derive(Deserialize, Serialize)]
pub struct AutoSubmitBatchMetadata {
    pub chain_id: u32,
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct BatchProofData {
    pub round_1_batch_ids: Option<Vec<AutoSubmitBatchMetadata>>,
    pub round_2_batch_ids: Option<Vec<AutoSubmitBatchMetadata>>,
    pub final_proof_batch_ids: Option<Vec<AutoSubmitBatchMetadata>>,
}

#[derive(Deserialize, Serialize)]
pub enum AutoSubmitStatus {
    Round1,
    Round2,
    Batched,
    RegisteredProof,
    Failed,
}

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub user_address: String,
    pub node_address: Option<String>,
    pub md5: String,
    pub task_type: String,
    pub status: TaskStatus,
    pub single_proof: Vec<u8>,
    pub proof: Vec<u8>,
    pub aux: Vec<u8>,
    pub shadow_instances: Vec<u8>,
    pub batch_instances: Vec<u8>,
    pub instances: Vec<u8>,
    pub public_inputs: Vec<String>,
    pub private_inputs: Vec<String>,
    pub input_context: Vec<u8>,
    pub input_context_type: Option<InputContextType>,
    pub output_context: Vec<u8>,
    pub _id: ObjectId,
    pub submit_time: String,
    pub process_started: Option<String>,
    pub process_finished: Option<String>,
    pub task_fee: Option<Vec<u8>>,
    pub status_message: Option<String>,
    pub internal_message: Option<String>,
    pub guest_statics: Option<u64>,
    pub task_verification_data: TaskVerificationData,
    pub debug_logs: Option<String>,
    pub proof_submit_mode: Option<ProofSubmitMode>,
    pub batch_proof_data: Option<BatchProofData>,
    pub auto_submit_status: Option<AutoSubmitStatus>,
    pub compression: Option<CompressionType>,
}

#[derive(Deserialize, Serialize)]
pub struct ConciseTask {
    pub _id: ObjectId,
    pub user_address: String,
    pub md5: String,
    pub task_type: String,
    pub status: TaskStatus,
    pub submit_time: String,
    pub process_started: Option<String>,
    pub process_finished: Option<String>,
    pub proof_submit_mode: Option<ProofSubmitMode>,
    pub auto_submit_status: Option<AutoSubmitStatus>,
}

#[derive(Deserialize, Serialize)]
pub struct TaskExternalHostTable {
    pub external_host_table: Vec<u8>,
    pub compression: Option<CompressionType>,
}

#[derive(Deserialize, Serialize)]
pub struct StaticFileVerificationData {
    pub static_file_checksum: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub enum AutoSubmitProofStatus {
    Pending,
    Batched,
    Failed,
}

#[derive(Deserialize, Serialize)]
pub struct AutoSubmitProof {
    pub _id: Option<ObjectId>,
    pub task_id: String,
    pub base_proof_circuit_size: u32,
    pub proof: Vec<u8>,
    pub batch_instances: Vec<u8>,
    pub shadow_instances: Option<Vec<u8>>,
    pub aux: Vec<u8>,
    pub batch_started: Option<String>,
    pub batch_finished: Option<String>,
    pub internal_message: Option<String>,
    pub static_files_verification_data: StaticFileVerificationData,
    pub auto_submit_network_chain_id: u32,
    pub status: AutoSubmitProofStatus,
}

#[derive(Deserialize, Serialize)]
pub enum Round1Status {
    Pending,
    Batched,
    Failed,
}

#[derive(Deserialize, Serialize)]
pub struct Round1Info {
    pub _id: Option<ObjectId>,
    pub round_1_ids: Vec<String>,
    pub task_ids: Vec<String>,
    pub target_instances: Vec<Vec<u8>>,
    pub proof: Vec<u8>,
    pub batch_instances: Vec<u8>,
    pub shadow_instances: Option<Vec<u8>>,
    pub aux: Vec<u8>,
    pub batch_started: Option<String>,
    pub batch_finished: Option<String>,
    pub internal_message: Option<String>,
    pub auto_submit_network_chain_id: u32,
    pub verifier_contracts: VerifierContracts,
    pub static_files_verification_data: StaticFileVerificationData,
    pub status: Round1Status,
}

#[derive(Deserialize, Serialize)]
pub enum Round2Status {
    ProofNotRegistered,
    ProofRegistered,
}

#[derive(Deserialize, Serialize)]
pub struct Round2Info {
    pub _id: Option<ObjectId>,
    pub round_2_ids: Vec<String>,
    pub task_ids: Vec<String>,
    pub target_instances: Vec<Vec<u8>>,
    pub proof: Vec<u8>,
    pub batch_instances: Vec<u8>,
    pub shadow_instances: Option<Vec<u8>>,
    pub aux: Vec<u8>,
    pub batched_time: Option<String>,
    pub internal_message: Option<String>,
    pub static_files_verification_data: StaticFileVerificationData,
    pub auto_submit_network_chain_id: u32,
    pub verifier_contracts: VerifierContracts,
    pub registered_tx_hash: Option<String>,
    pub status: Round2Status,
}

#[derive(Deserialize, Serialize)]
pub struct PaginationParams {
    pub total: Option<u64>,
    pub start: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct PaginatedQuery<T> {
    #[serde(flatten)]
    pub query: T,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Deserialize, Serialize)]
pub struct AutoSubmitProofQuery {
    pub id: Option<String>,
    pub task_id: Option<String>,
    pub status: Option<AutoSubmitProofStatus>,
    pub circuit_size: Option<u32>,
    pub chain_id: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct Round1InfoQuery {
    pub id: Option<String>,
    pub task_id: Option<String>,
    pub status: Option<Round1Status>,
    pub circuit_size: Option<u32>,
    pub chain_id: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct Round2InfoQuery {
    pub id: Option<String>,
    pub round_2_id: Option<String>,
    pub task_id: Option<String>,
    pub status: Option<Round2Status>,
    pub chain_id: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub enum TaskType {
    Setup,
    Prove,
    Reset,
}

#[derive(Deserialize, Serialize)]
pub enum ImageStatus {
    Received,
    Initialized,
    Verified,
}

#[derive(Deserialize, Serialize)]
pub enum ProvePaymentSrc {
    Default,
    CreatorPay,
}

#[derive(Deserialize, Serialize)]
pub enum AddProveTaskRestrictions {
    Anyone,
    CreatorOnly,
}

#[derive(Deserialize, Serialize)]
pub struct BaseAddImageParams {
    pub name: String,
    pub image: Vec<u8>,
    pub image_md5: String,
    pub user_address: String,
    pub description_url: String,
    pub avator_url: String,
    pub circuit_size: u32,
    pub prove_payment_src: ProvePaymentSrc,
    pub auto_submit_network_ids: Vec<u32>,
    pub add_prove_task_restrictions: Option<AddProveTaskRestrictions>,
    pub inherited_merkle_data_md5: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct WithInitialContext {
    pub initial_context: Vec<u8>,
    pub initial_context_md5: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum InitialContext {
    With(WithInitialContext),
    Without,
}

#[derive(Deserialize, Serialize)]
pub struct AddImageParams {
    #[serde(flatten)]
    pub base: BaseAddImageParams,
    #[serde(flatten)]
    pub context: InitialContext,
}

#[derive(Deserialize, Serialize)]
pub struct BaseProvingParams {
    pub user_address: String,
    pub md5: String,
    pub public_inputs: Vec<String>,
    pub private_inputs: Vec<String>,
    pub proof_submit_mode: ProofSubmitMode,
}

#[derive(Deserialize, Serialize)]
pub struct WithCustomInputContext {
    pub input_context_type: InputContextType,
    pub input_context: Vec<u8>,
    pub input_context_md5: String,
}

#[derive(Deserialize, Serialize)]
pub struct WithNonCustomInputContext {
    #[serde(default)]
    pub input_context_type: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum CustomContext {
    With(WithCustomInputContext),
    WithNonCustom(WithNonCustomInputContext),
    Without,
}

#[derive(Deserialize, Serialize)]
pub struct ProvingParams {
    #[serde(flatten)]
    pub base: BaseProvingParams,
    #[serde(flatten)]
    pub context: CustomContext,
}

#[derive(Deserialize, Serialize)]
pub struct DeployParams {
    pub user_address: String,
    pub md5: String,
    pub chain_id: u32,
}

#[derive(Deserialize, Serialize)]
pub struct BaseResetImageParams {
    pub md5: String,
    pub circuit_size: u32,
    pub user_address: String,
    pub prove_payment_src: ProvePaymentSrc,
    pub auto_submit_network_ids: Vec<u32>,
    pub add_prove_task_restrictions: Option<AddProveTaskRestrictions>,
}

#[derive(Deserialize, Serialize)]
pub struct WithResetContext {
    pub reset_context: Vec<u8>,
    pub reset_context_md5: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResetContext {
    With(WithResetContext),
    Without,
}

#[derive(Deserialize, Serialize)]
pub struct ResetImageParams {
    #[serde(flatten)]
    pub base: BaseResetImageParams,
    #[serde(flatten)]
    pub context: ResetContext,
}

#[derive(Deserialize, Serialize)]
pub struct ModifyImageParams {
    pub md5: String,
    pub user_address: String,
    pub description_url: String,
    pub avator_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct WithSignature {
    pub signature: String,
}

#[derive(Deserialize, Serialize)]
pub struct WithoutSignature;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Signature {
    With(WithSignature),
    Without(WithoutSignature),
}

#[derive(Deserialize, Serialize)]
pub struct SignaturedRequest<T> {
    #[serde(flatten)]
    pub base: T,
    #[serde(flatten)]
    pub signature: Signature,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyData {
    pub proof: Vec<Vec<u8>>,
    pub target_instances: Vec<Vec<u8>>,
    pub aggregator_instances: Vec<Vec<u8>>,
    pub aux_instances: Vec<Vec<u8>>,
}

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    pub user_address: Option<String>,
    pub md5: Option<String>,
    pub id: Option<String>,
    pub tasktype: Option<String>,
    pub taskstatus: Option<String>,
    pub start: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct TaskExternalHostTableParams {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyProofParams {
    pub aggregate_proof: Vec<u8>,
    pub verify_instance: Vec<u8>,
    pub aux: Vec<u8>,
    pub instances: Vec<Vec<u8>>,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyBatchProofParams {
    pub membership_proof_index: Vec<Vec<u8>>,
    pub verify_instance: Vec<u8>,
    pub sibling_instances: Vec<Vec<u8>>,
    pub round_1_shadow_instance: Vec<u8>,
    pub target_instances: Vec<Vec<u8>>,
}

#[derive(Deserialize, Serialize)]
pub struct LogQuery {
    pub id: String,
    pub user_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct TaskFeeList {
    pub setup_fee: String,
    pub prove_fee: String,
    pub auto_submit_prove_fee_per_network: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChainInfo {
    pub chain_id: u32,
    pub chain_name: String,
    pub block_explorer_url: String,
    pub deploy_fee: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenParams {
    pub token_address: String,
    pub network_id: u32,
    pub topup_conversion_rate: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct TokenData {
    pub decimals: u64,
    pub symbol: String,
}

#[derive(Deserialize, Serialize)]
pub struct ContractDeployments {
    pub chain_id: u32,
    pub circuit_size: u32,
    pub aggregator_lib_address: String,
    pub aggregator_config_address: String,
    pub aggregator_verifier_steps: Vec<String>,
    pub aggregator_verifier: String,
    pub batch_verifier: String,
    pub static_file_checksum: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub enum SubscriptionType {
    Basic,
    Developer,
    Enterprise,
}

#[derive(Deserialize, Serialize)]
pub enum BaseSubscriptionDuration {
    Month,
    Year,
}

#[derive(Deserialize, Serialize)]
pub struct SubscriptionDuration {
    pub base_duration: BaseSubscriptionDuration,
    pub multiplier: u64,
}

#[derive(Deserialize, Serialize)]
pub struct SubscriptionParams {
    pub subscription_type: SubscriptionType,
    pub duration: SubscriptionDuration,
    pub token_params: TokenParams,
    pub token_data: TokenData,
    pub price_per_base_duration: String,
    pub credited_amount: String,
    pub enabled: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ServerVersionInfo {
    pub current_version: String,
    pub minimum_supported_node_version: String,
}

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub receiver_address: String,
    pub deployer_address: String,
    pub task_fee_list: TaskFeeList,
    pub chain_info_list: Vec<ChainInfo>,
    pub latest_server_checksum: Vec<u8>,
    pub topup_token_params: TokenParams,
    pub topup_token_data: TokenData,
    pub deployments: Vec<ContractDeployments>,
    pub subscription_plans: Vec<SubscriptionParams>,
    pub supported_auto_submit_network_ids: Vec<u32>,
    pub server_version_info: ServerVersionInfo,
}

#[derive(Deserialize, Serialize)]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ChainDetails {
    pub chainHexId: String,
    pub chainName: String,
    pub nativeCurrency: NativeCurrency,
    pub rpcUrls: Vec<String>,
    pub blockExplorerUrls: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DeploymentInfo {
    pub chain_id: u32,
    pub address: String,
}

#[derive(Deserialize, Serialize)]
pub struct InheritedMerkleDataInfo {
    pub md5: String,
}

#[derive(Deserialize, Serialize)]
pub struct ImageChecksum {
    pub x: Vec<u8>,
    pub y: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct Image {
    pub user_address: String,
    pub md5: String,
    pub deployment: Vec<DeploymentInfo>,
    pub description_url: String,
    pub avator_url: String,
    pub circuit_size: u32,
    pub context: Option<Vec<u8>>,
    pub initial_context: Option<Vec<u8>>,
    pub status: String,
    pub checksum: Option<ImageChecksum>,
    pub prove_payment_src: ProvePaymentSrc,
    pub auto_submit_network_ids: Vec<u32>,
    pub inherited_merkle_data_info: Option<InheritedMerkleDataInfo>,
    pub add_prove_task_restrictions: AddProveTaskRestrictions,
}

#[derive(Deserialize, Serialize)]
pub struct PaymentParams {
    pub txhash: String,
}

#[derive(Deserialize, Serialize)]
pub struct SubscriptionRequest {
    pub subscriber_address: String,
    pub subscription_type: SubscriptionType,
    pub duration: SubscriptionDuration,
    pub payment_hash: String,
}

#[derive(Deserialize, Serialize)]
pub enum SubscriptionStatus {
    Active,
    Expired,
}

#[derive(Deserialize, Serialize)]
pub struct ERC20DepositInfo {
    pub user_address: String,
    pub receiver_address: String,
    pub txhash: String,
    pub amount: String,
    pub token_params: TokenParams,
    pub token_data: TokenData,
}

#[derive(Deserialize, Serialize)]
pub struct Subscription {
    pub subscriber_address: String,
    pub start_date: u64,
    pub end_date: u64,
    pub params: SubscriptionParams,
    pub status: SubscriptionStatus,
    pub payment_details: Vec<ERC20DepositInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct UserQueryParams {
    pub user_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct TxHistoryQueryParams {
    pub user_address: String,
    pub start: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub user_address: String,
    #[deprecated]
    pub balance: Vec<u8>,
    pub credits: String,
    pub credit_deficit: String,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionInfo {
    pub txhash: String,
    pub value: Vec<u8>,
    pub user_address: String,
    pub receiver_address: String,
}

#[derive(Deserialize, Serialize)]
pub enum MaintenanceModeType {
    Disabled,
    Enabled,
}

#[derive(Deserialize, Serialize)]
pub enum AdminRequestType {
    Default,
    MaintenanceMode,
    ArchiveOperation,
    ForceTaskToReprocess,
}

#[derive(Deserialize, Serialize)]
pub struct SetMaintenanceModeParams {
    pub mode: MaintenanceModeType,
    pub nonce: u64,
    pub request_type: AdminRequestType,
    pub user_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct EstimatedProofFeeParams {
    pub user_address: String,
    pub md5: String,
    pub proof_submit_mode: ProofSubmitMode,
}

#[derive(Deserialize, Serialize)]
pub struct EstimatedProofFee {
    pub min: Option<u64>,
    pub max: Option<u64>,
    pub msg: String,
}

#[derive(Deserialize, Serialize)]
pub struct ForceUnprovableToReprocessParams {
    pub task_ids: Vec<String>,
    pub nonce: u64,
    pub request_type: AdminRequestType,
    pub user_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct ForceDryrunFailsToReprocessParams {
    pub task_ids: Vec<String>,
    pub nonce: u64,
    pub request_type: AdminRequestType,
    pub user_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProverNodeTimeRangeStatsParams {
    pub address: String,
    pub start_ts: String,
    pub end_ts: String,
}

#[derive(Deserialize, Serialize)]
pub struct RangeStats {
    pub successful: u64,
    pub failed: u64,
    pub timed_out: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ProverNodeTimeRangeStats {
    pub fst_id: Option<String>,
    pub fst_ts: Option<String>,
    pub lst_id: Option<String>,
    pub lst_ts: Option<String>,
    pub stats: RangeStats,
}

#[derive(Deserialize, Serialize)]
pub struct VolumeRange {
    pub n_records: u64,
    pub fst: ObjectId,
    pub lst: ObjectId,
    pub fst_ts: String,
    pub lst_ts: String,
}

#[derive(Deserialize, Serialize)]
pub struct ArchiveVolumeMetadata {
    pub version: String,
    pub volume_name: String,
    pub original_coll_name: String,
    pub range: Option<VolumeRange>,
    pub _id: ObjectId,
    pub prev_last_ts: String,
    pub image_md5s: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct VolumeListQuery {
    pub start: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct VolumeDetailQuery {
    pub tasks_start: Option<u64>,
    pub tasks_limit: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct VolumeDetailResponse {
    pub volume: ArchiveVolumeMetadata,
    pub tasks: PaginationResult<ConciseTask>,
}

#[derive(Deserialize, Serialize)]
pub struct ArchiveQuery {
    pub task_id: Option<String>,
    pub md5: Option<String>,
    pub start_timestamp: Option<String>,
    pub end_timestamp: Option<String>,
    pub start: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct ArchiveMetadataOverview {
    pub first: Option<ArchiveVolumeMetadata>,
    pub last: Option<ArchiveVolumeMetadata>,
}

#[derive(Deserialize, Serialize)]
pub struct ArchivedFinalProofNetworkInfo {
    pub verifier_contracts: VerifierContracts,
}

#[derive(Deserialize, Serialize)]
pub struct ArchivedFinalBatchProof {
    pub _id: ObjectId,
    pub original_final_proof_id: String,
    pub included_md5s: Vec<String>,
    pub round_2_ids: Vec<String>,
    pub round_1_ids: Vec<String>,
    pub task_ids: Vec<String>,
    pub target_instances: Vec<Vec<u8>>,
    pub proof: Vec<u8>,
    pub batch_instances: Vec<u8>,
    pub shadow_instances: Vec<u8>,
    pub aux: Vec<u8>,
    pub round_1_proof: Vec<Vec<u8>>,
    pub round_1_batch_instances: Vec<Vec<u8>>,
    pub round_1_shadow_instances: Vec<Vec<u8>>,
    pub round_1_aux: Vec<Vec<u8>>,
    pub round_1_target_instances: Vec<Vec<u8>>,
    pub batched_time: String,
    pub internal_message: Option<String>,
    pub static_files_verification_data: Option<StaticFileVerificationData>,
    pub auto_submit_network_chain_id: u32,
    pub verifier_contracts: VerifierContracts,
    pub registered_tx_hash: String,
    pub status: Round2Status,
}

#[derive(Deserialize, Serialize)]
pub struct ChainConfig {
    pub chain_id: u32,
    pub chain_name: String,
    pub deploy_strategy: Option<String>,
    pub block_explorer_url: String,
    rpc_url: String,
    pub native_currency: NativeCurrency,
    pub deploy_fee: String,
}

#[derive(Deserialize, Serialize)]
pub struct ArchiveConfig {
    pub processing_tasks_limit: u64,
    pub processing_time_limit_mins: u64,
    pub archive_cron_schedule: String,
    pub cleanup_cron_schedule: String,
    pub scheduled_archive_days_behind: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ArchiveServerConfig {
    pub prod_mongodb_uri: String,
    pub archive_mongodb_uri: String,
    pub volume_dump_dir: String,
    pub network_list: Vec<ChainConfig>,
    pub archive_config: ArchiveConfig,
}

#[derive(Deserialize, Serialize)]
pub struct AddTaskResult {
    pub md5: String,
    pub id: String,
}
