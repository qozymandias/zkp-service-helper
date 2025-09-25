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
use crate::interface::ProverNodeTimeRangeStatsParams;
use crate::interface::ProvingParams;
use crate::interface::ResetImageParams;
use crate::interface::SetMaintenanceModeParams;
use crate::interface::SubscriptionRequest;

/// Defines customisation hooks for controlling how request payloads are serialized.
///
/// Types that implement [`SerializationAttributes`] can influence how fields are treated during serialization, such as
/// marking specific fields as raw bytes, ignoring certain fields, or transforming the serialized structure into a
/// canonical string representation.
///
/// This trait is used in POST request building to provide fine-grained control over how objects are turned into messages
/// before being signed or transmitted.
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

    fn requires_json_body() -> bool {
        false
    }
}

/// Serializes an object into its canonical message form and signs it with the given private key.
///
/// This function uses the [`SerializationAttributes`] implementation of the type `T` to determine how the object is
/// transformed into a string message (via [`SerializationAttributes::create_message`]). It then signs the resulting
/// message with the provided Ethereum private key using [`ethers::signers::LocalWallet`].
///
/// # Type Parameters
///
/// - `T`: The type of the object to be signed. Must implement [`Serialize`] and [`SerializationAttributes`].
///
/// # Arguments
///
/// - `obj`: The object to serialize and sign.
/// - `private_key`: The private key in hex string format, used to sign the message.
///
/// # Returns
///
/// Returns [`anyhow::Result`] containing the signature as a hex-encoded string on success,
/// or an error if serialization, key parsing, or signing fails.
///
/// # Errors
///
/// This function will return an error if:
///
/// - Serialization of `obj` into a canonical message fails.
/// - The `private_key` string cannot be parsed into an [`ethers::signers::LocalWallet`].
/// - The signing operation fails.
pub async fn sign_object<T: Serialize + SerializationAttributes>(
    obj: &T,
    private_key: String,
) -> anyhow::Result<String> {
    let message = T::create_message(obj)?;
    let wallet = private_key.parse::<ethers::signers::LocalWallet>()?;
    Ok(wallet.sign_message(message).await.map(|s| s.to_string())?)
}

/// Utility for converting Serializable objects into [`reqwest::multipart::Form`]s.
///
/// `IntoMultipartForm` provides methods for converting `Serialize` + [`SerializationAttributes`] objects into multipart
/// form data, which can be used in HTTP POST/PUT requests. It handles primitive values, arrays, and byte fields while
/// rejecting unsupported nested JSON structures.
pub struct IntoMultipartForm;

impl IntoMultipartForm {
    fn into_part(obj: serde_json::Value) -> anyhow::Result<Part> {
        Ok(match obj {
            Value::Bool(v) => Part::text(v.to_string()),
            Value::Number(v) => Part::text(v.to_string()),
            Value::String(v) => Part::text(v),
            _ => return Err(anyhow::anyhow!("Must be primitive object type")),
        })
    }

    fn create_parts<T: SerializationAttributes>(name: &str, obj: serde_json::Value) -> anyhow::Result<Vec<Part>> {
        Ok(match obj {
            Value::Null => vec![],
            Value::Bool(_) | Value::Number(_) | Value::String(_) => vec![Self::into_part(obj)?],
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
                    vec![Part::bytes(bytes)]
                } else {
                    vs.into_iter().map(Self::into_part).collect::<anyhow::Result<Vec<_>>>()?
                }
            }
        })
    }

    /// Converts a serializable object into a [`reqwest::multipart::Form`].
    ///
    /// The object must serialize into a JSON object (top-level map). Each field is converted into one or more [`Part`]s
    /// using [`Self::create_parts`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - Serialization fails.
    /// - The top-level value is not an object.
    /// - Any field fails conversion in [`Self::create_parts`].
    pub fn into_multipart_form<T: Serialize + SerializationAttributes>(
        params: T,
    ) -> anyhow::Result<reqwest::multipart::Form> {
        if let Value::Object(map) = serde_json::to_value(params)? {
            let mut form = reqwest::multipart::Form::new();
            for (k, v) in map {
                let parts = Self::create_parts::<T>(&k, v)?;
                for part in parts {
                    form = form.part(k.clone(), part);
                }
            }
            Ok(form)
        } else {
            Err(anyhow::anyhow!("Top level of json must be Object type"))
        }
    }
}

impl SerializationAttributes for ProvingParams {
    fn fields_to_ignore() -> Vec<String> {
        vec!["input_context".to_string()]
    }
}

impl SerializationAttributes for DeployParams {}

impl SerializationAttributes for ResetImageParams {}

impl SerializationAttributes for ModifyImageParams {
    fn create_message<T: Serialize>(obj: &T) -> anyhow::Result<String> {
        serde_json::to_string(obj).map_err(Into::into)
    }
}

impl SerializationAttributes for SetMaintenanceModeParams {}

impl SerializationAttributes for ForceUnprovableToReprocessParams {}

impl SerializationAttributes for ForceDryrunFailsToReprocessParams {}

impl SerializationAttributes for SubscriptionRequest {}

impl SerializationAttributes for PaymentParams {}

impl SerializationAttributes for LogQuery {
    fn create_message<T: Serialize>(obj: &T) -> anyhow::Result<String> {
        serde_json::to_string(obj).map_err(Into::into)
    }

    fn requires_json_body() -> bool {
        true
    }
}

impl SerializationAttributes for ProverNodeTimeRangeStatsParams {
    fn create_message<T: Serialize>(obj: &T) -> anyhow::Result<String> {
        serde_json::to_string(obj).map_err(Into::into)
    }

    fn requires_json_body() -> bool {
        true
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
