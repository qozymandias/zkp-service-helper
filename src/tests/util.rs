use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub(super) struct DetailsConfig {
    pub server_url: String,
    pub private_key: String,
    pub chain_id: u32,
    pub pedantic_checks: bool,
}

#[derive(Deserialize, Serialize)]
pub(super) struct VerifyConfig {
    pub provider_url: String,
    pub manual_task_id_to_verify: String,
}

#[derive(Deserialize, Serialize)]
pub(super) struct QueryConfig {
    pub task_id: String,
    pub md5: String,
    pub node_address: String,
}

#[derive(Deserialize, Serialize)]
pub(super) struct AutoSubmitConfig {
    pub round1_id: String,
    pub round2_id: String,
    pub task_id_in_auto_submit_batch: String,
}

#[derive(Deserialize, Serialize)]
pub(super) struct ArchiveConfig {
    pub server_url: String,
    pub id: String,
    pub archived_task_id: String,
    pub archive_volume_name: String,
    pub archive_auto_submit_volume_name: String,
}

#[derive(Deserialize, Serialize)]
pub(super) struct TasksConfig {
    pub image: String,
}

#[derive(Deserialize, Serialize)]
pub(super) struct TestConfig {
    pub details: DetailsConfig,
    pub verify: VerifyConfig,
    pub query: QueryConfig,
    pub auto_submit: AutoSubmitConfig,
    pub archive: ArchiveConfig,
    pub tasks: TasksConfig,
}

impl TestConfig {
    fn read_config() -> anyhow::Result<Self> {
        let file = std::fs::File::open("test.json")?;
        let reader = std::io::BufReader::new(file);
        let data: Self = serde_json::from_reader(reader)?;
        Ok(data)
    }

    pub fn init() -> Self {
        Self::read_config().expect("Should be able to read test config")
    }

    pub fn user_address(&self) -> String {
        use ethers::signers::Signer;
        let wallet: ethers::signers::LocalWallet = self.details.private_key.parse().expect("Private key should parse");
        format!("{:#x}", wallet.address())
    }
}

pub(super) fn check_and_print<T: for<'de> Deserialize<'de> + Serialize>(result: anyhow::Result<T>) -> T {
    result
        .inspect(|inp| {
            let s = serde_json::to_string(&inp).expect("Should be serializable");
            let _ = serde_json::from_str::<T>(&s).expect("Should be deserializable");
            println!("{s}");
        })
        .expect("Result should be valid")
}
