use ethers::signers::Signer;
use reqwest::multipart::Part;
use serde::Serialize;
use serde_json::Value;

use crate::interface::AddImageParams;
use crate::interface::DeployParams;
use crate::interface::ForceDryrunFailsToReprocessParams;
use crate::interface::ForceUnprovableToReprocessParams;
use crate::interface::LogQuery;
use crate::interface::ModifyImageParams;
use crate::interface::PaymentParams;
use crate::interface::ProvingParams;
use crate::interface::ResetImageParams;
use crate::interface::SetMaintenanceModeParams;
use crate::interface::SubscriptionRequest;

pub trait SerializationAttributes {
    #[must_use]
    fn fields_which_are_bytes() -> Vec<String> {
        vec![]
    }

    #[must_use]
    fn fields_to_ignore() -> Vec<String> {
        vec![]
    }

    fn create_message<T: Serialize>(obj: &T) -> anyhow::Result<String> {
        Ok(match serde_json::to_value(obj)? {
            Value::Null => String::new(),
            Value::Bool(v) => v.to_string(),
            Value::Number(v) => v.to_string(),
            Value::String(v) => v,
            Value::Array(vs) => {
                let mut message = String::new();
                for v in vs {
                    message.push_str(&Self::create_message(&v)?);
                }
                message
            }
            Value::Object(map) => {
                let mut message = String::new();
                for (k, v) in map {
                    if !Self::fields_to_ignore().contains(&k) {
                        message.push_str(&Self::create_message(&v)?);
                    }
                }
                message
            }
        })
    }
}

impl SerializationAttributes for ProvingParams {}

impl SerializationAttributes for DeployParams {}

impl SerializationAttributes for ResetImageParams {}

impl SerializationAttributes for ModifyImageParams {}

impl SerializationAttributes for SetMaintenanceModeParams {}

impl SerializationAttributes for ForceUnprovableToReprocessParams {}

impl SerializationAttributes for ForceDryrunFailsToReprocessParams {}

impl SerializationAttributes for SubscriptionRequest {}

impl SerializationAttributes for PaymentParams {}

impl SerializationAttributes for LogQuery {
    fn create_message<T: Serialize>(obj: &T) -> anyhow::Result<String> {
        serde_json::to_string(obj).map_err(Into::into)
    }
}

impl SerializationAttributes for AddImageParams {
    fn fields_which_are_bytes() -> Vec<String> {
        vec!["image".to_string()]
    }
    fn fields_to_ignore() -> Vec<String> {
        vec!["image".to_string(), "initial_context".to_string()]
    }
}

pub async fn sign_object<T: Serialize + SerializationAttributes>(
    obj: &T,
    private_key: String,
) -> anyhow::Result<String> {
    let message = T::create_message(obj)?;
    let wallet = private_key.parse::<ethers::signers::LocalWallet>()?;
    Ok(wallet.sign_message(message).await.map(|s| s.to_string())?)
}

fn create_part<T: SerializationAttributes>(name: &str, obj: serde_json::Value) -> anyhow::Result<Part> {
    Ok(match obj {
        Value::Null => Part::text(String::new()),
        Value::Bool(v) => Part::text(v.to_string()),
        Value::Number(v) => Part::text(v.to_string()),
        Value::String(v) => Part::text(v),
        Value::Object(_) => return Err(anyhow::anyhow!("Nested json objects not supported")),
        Value::Array(vs) => {
            if T::fields_which_are_bytes().contains(&name.to_string()) {
                let mut bytes = Vec::new();
                for v in vs {
                    if let serde_json::Value::Number(n) = v {
                        bytes.push(
                            n.as_u64()
                                .and_then(|v| u8::try_from(v).ok())
                                .ok_or_else(|| anyhow::anyhow!("Byte must be u8 type"))?,
                        );
                    } else {
                        return Err(anyhow::anyhow!("Bytes field can only have Number values"));
                    }
                }
                Part::bytes(bytes)
            } else {
                Part::text(vs.iter().map(ToString::to_string).collect::<String>())
            }
        }
    })
}

pub fn into_multipart_form<T: Serialize + SerializationAttributes>(
    params: T,
) -> anyhow::Result<reqwest::multipart::Form> {
    if let Value::Object(map) = serde_json::to_value(params)? {
        let mut form = reqwest::multipart::Form::new();
        for (k, v) in map {
            let part = create_part::<T>(&k, v)?;
            form = form.part(k, part);
        }
        Ok(form)
    } else {
        Err(anyhow::anyhow!("Top level of json must be Object type"))
    }
}
