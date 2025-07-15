use ethers::signers::Signer;
use serde::Serialize;

pub async fn sign_object<T: Serialize>(obj: &T, private_key: String) -> anyhow::Result<String> {
    let message = serde_json::to_string(obj)?;
    let wallet = private_key.parse::<ethers::signers::LocalWallet>()?;
    Ok(wallet.sign_message(message).await.map(|s| s.to_string())?)
}
