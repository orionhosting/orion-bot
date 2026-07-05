use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

use crate::{
    ApiState, ApiStatus, CreateCreditTransactionBody, CreditTransactionResult, OrionUser,
    PatchStateBody,
};

#[derive(Debug, Error)]
pub enum OrionError {
    #[error("API error {status}{}", .message.as_deref().map(|m| format!(": {}", m)).unwrap_or_default())]
    Api {
        status: u16,
        message: Option<String>,
    },
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

pub struct OrionApiClient {
    base_url: String,
    http: Arc<reqwest::Client>,
    api_key: String,
}

impl OrionApiClient {
    pub fn new(api_key: String, http: Arc<reqwest::Client>) -> Self {
        Self {
            base_url: "https://api.orionhost.xyz".to_string(),
            api_key,
            http,
        }
    }

    fn api_url(&self, path: &str) -> String {
        format!("{}/api/{}", self.base_url, path)
    }

    async fn handle_error_response(resp: reqwest::Response) -> OrionError {
        let status = resp.status();
        let status_code = status.as_u16();
        let message = resp
            .json::<serde_json::Value>()
            .await
            .ok()
            .and_then(|v| v.get("message").and_then(|m| m.as_str()).map(str::to_owned));

        OrionError::Api {
            status: status_code,
            message,
        }
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T, OrionError> {
        let resp = self
            .http
            .get(self.api_url(path))
            .header("Authorization", &self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(Self::handle_error_response(resp).await);
        }

        Ok(resp.json::<T>().await?)
    }

    async fn post<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, OrionError> {
        let resp = self
            .http
            .post(self.api_url(path))
            .header("Authorization", &self.api_key)
            .json(body)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(Self::handle_error_response(resp).await);
        }

        Ok(resp.json::<T>().await?)
    }

    async fn patch_void<B: Serialize>(&self, path: &str, body: &B) -> Result<(), OrionError> {
        let resp = self
            .http
            .patch(self.api_url(path))
            .header("Authorization", &self.api_key)
            .json(body)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(Self::handle_error_response(resp).await);
        }

        Ok(())
    }
}

impl OrionApiClient {
    pub async fn get_status(&self) -> Result<ApiStatus, OrionError> {
        self.get("status").await
    }

    pub async fn get_state(&self) -> Result<ApiState, OrionError> {
        self.get("state").await
    }

    pub async fn patch_state(&self, body: PatchStateBody) -> Result<(), OrionError> {
        self.patch_void("state", &body).await
    }

    pub async fn get_user(&self, discord_id: impl AsRef<str>) -> Result<OrionUser, OrionError> {
        self.get(&format!("users/{}", discord_id.as_ref())).await
    }

    pub async fn create_credit_transaction(
        &self,
        discord_id: impl AsRef<str>,
        body: CreateCreditTransactionBody,
    ) -> Result<CreditTransactionResult, OrionError> {
        self.post(&format!("users/{}/credits", discord_id.as_ref()), &body)
            .await
    }
}
