use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

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
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    fn to_body<T: Serialize>(value: &T) -> Result<reqwest::Body, serde_json::Error> {
        serde_json::to_vec(value).map(reqwest::Body::from)
    }

    fn to_path(&self, path: TaskEndpoint) -> String {
        format!("{}/{}{}", self.endpoint, path.as_path(), path.path_params())
    }

    async fn request<U: Serialize, V: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: TaskEndpoint,
        payload: U,
        signature: Option<String>,
    ) -> anyhow::Result<V> {
        let client = reqwest::Client::new();
        let mut req = client
            .request(method, self.to_path(path))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(Self::to_body(&payload)?);

        if let Some(sig) = signature {
            req = req.header("x-eth-signature", sig);
        }
        Ok(client.execute(req.build()?).await?.json::<V>().await?)
    }

    pub async fn get<U: Serialize, V: for<'de> Deserialize<'de> + Serialize>(
        &self,
        path: TaskEndpoint,
        params: U,
        signature: Option<String>,
    ) -> anyhow::Result<V> {
        self.request::<_, RequestResult<V>>(reqwest::Method::GET, path, params, signature)
            .await
            .map(|res| res.result)
    }

    pub async fn post<U: Serialize, V: for<'de> Deserialize<'de> + Serialize>(
        &self,
        path: TaskEndpoint,
        params: U,
        signature: Option<String>,
    ) -> anyhow::Result<V> {
        self.request::<_, RequestResult<V>>(reqwest::Method::POST, path, params, signature)
            .await
            .map(|res| res.result)
    }
}
