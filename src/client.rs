use reqwest::{Client as ReqwestClient, Error, Response};
use serde::Serialize;
use serde_json::Value;

pub const BASE_URL: &str = "https://us-central1-stocktalker-app.cloudfunctions.net/api/v1";
// We default to the live function. If a custom domain is mapped like api.stocktalkerai.com, change this here.

pub struct Client {
    http: ReqwestClient,
    api_key: Option<String>,
}

impl Client {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            http: ReqwestClient::new(),
            api_key,
        }
    }

    fn maybe_auth(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(key) = &self.api_key {
            builder.header("Authorization", format!("Bearer {}", key))
        } else {
            builder
        }
    }

    pub async fn get(&self, path: &str) -> Result<Value, anyhow::Error> {
        let url = format!("{}{}", BASE_URL, path);
        let builder = self.http.get(&url);
        let res = self.maybe_auth(builder).send().await?;
        self.handle_response(res).await
    }

    pub async fn post<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<Value, anyhow::Error> {
        let url = format!("{}{}", BASE_URL, path);
        let builder = self.http.post(&url).json(body);
        let res = self.maybe_auth(builder).send().await?;
        self.handle_response(res).await
    }

    pub async fn delete(&self, path: &str) -> Result<Value, anyhow::Error> {
        let url = format!("{}{}", BASE_URL, path);
        let builder = self.http.delete(&url);
        let res = self.maybe_auth(builder).send().await?;
        self.handle_response(res).await
    }

    pub async fn patch<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<Value, anyhow::Error> {
        let url = format!("{}{}", BASE_URL, path);
        let builder = self.http.patch(&url).json(body);
        let res = self.maybe_auth(builder).send().await?;
        self.handle_response(res).await
    }

    async fn handle_response(&self, res: Response) -> Result<Value, anyhow::Error> {
        let status = res.status();
        let payload: Value = res.json().await?;

        if !status.is_success() {
            // Assume the error is embedded loosely based on our envelope.
            return Err(anyhow::anyhow!("API Error: {}", payload));
        }

        // Return the raw payload, we usually expect a `{"data": ...}` envelope.
        Ok(payload)
    }
}
