use serde::Deserialize;
use serde::Serialize;

use super::util::into_multipart_form;
use super::util::SerializationAttributes;
use super::TaskEndpoint;

#[derive(Deserialize, Serialize)]
pub struct RequestResult<T: Serialize> {
    pub success: bool,
    pub result: T,
}

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

    pub async fn post<U: Serialize + SerializationAttributes, V: for<'de> Deserialize<'de> + Serialize>(
        &self,
        path: TaskEndpoint,
        params: U,
        signature: Option<String>,
    ) -> anyhow::Result<V> {
        let url = self.to_path(&path);
        println!("POST {url}");

        let req = reqwest::Client::new().post(url).multipart(into_multipart_form(params)?);
        Self::execute(req, signature).await
    }

    pub async fn post_json_body<U: Serialize + SerializationAttributes, V: for<'de> Deserialize<'de> + Serialize>(
        &self,
        path: TaskEndpoint,
        params: U,
        signature: Option<String>,
    ) -> anyhow::Result<V> {
        let url = self.to_path(&path);
        println!("POST {url}");

        let req = reqwest::Client::new().post(url).json(&params);
        Self::execute(req, signature).await
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
}
