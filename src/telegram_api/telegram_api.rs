use reqwest::{Client, Response};
use serde_json::json;

use crate::{env_config::EnvConfig, types::Result};
use std::sync::Arc;
#[derive(Clone)]
pub struct TelegramApi {
    pub tg_api_url: String,
    pub token: String,
    pub client: Arc<Client>,
}

impl TelegramApi {
    pub fn new(env_h: Arc<EnvConfig>, client: Arc<Client>) -> Self {
        TelegramApi {
            tg_api_url: env_h.tg_api_url.to_string(),
            token: env_h.tg_token.to_string(),
            client: client,
        }
    }

    pub async fn send_message(&self, chat_id: &i64, text: &str) -> Result<Response> {
        let send_message_url = format!("{}/bot{}/sendMessage", self.tg_api_url, self.token);
        let response = self
            .client
            .post(send_message_url)
            .json(&json!({ "chat_id": chat_id, "text": text }))
            .send()
            .await?;
        Ok(response)
    }

    pub async fn get_updates(&self, offset: i64) -> Result<Response> {
        let send_message_url = &format!("{}/bot{}/getUpdates", self.tg_api_url, self.token);
        let response = self
            .client
            .post(send_message_url)
            .query(&[("offset", offset), ("timeout", 30_i64)])
            .send()
            .await?;
        Ok(response)
    }

    pub async fn set_webhook(&self, url: &str) -> Result<Response> {
        let send_message_url = &format!("{}/bot{}/setWebhook", self.tg_api_url, self.token);
        let response = self
            .client
            .post(send_message_url)
            .json(&json!({ "url": url }))
            .send()
            .await?;
        Ok(response)
    }

    pub async fn delete_webhook(&self) -> Result<Response> {
        let delete_webhook_url = format!("{}/bot{}/deleteWebhook", self.tg_api_url, self.token);
        let response = self.client.post(delete_webhook_url).send().await?;
        Ok(response)
    }
}
