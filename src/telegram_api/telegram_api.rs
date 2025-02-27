use reqwest::{Client, Response};
use serde_json::json;

use crate::env_config::EnvConfig;

#[derive(Clone)]
pub struct TelegramApi {
    pub tg_api_url: String,
    pub token: String,
    pub client: Client,
}

impl TelegramApi {
    pub fn new(env_h: &EnvConfig) -> Self {
        TelegramApi {
            tg_api_url: env_h.tg_api_url.to_string(),
            token: env_h.token.to_string(),
            client: Client::new(),
        }
    }

    pub async fn send_message(&self, chat_id: &i64, text: &str) -> reqwest::Result<Response> {
        let send_message_url = format!("{}/bot{}/sendMessage", self.tg_api_url, self.token);
        let response = self
            .client
            .post(send_message_url)
            .json(&json!({ "chat_id": chat_id, "text": text }))
            .send()
            .await?;
        Ok(response)
    }

    pub async fn set_webhook(&self, url: &str) -> reqwest::Result<Response> {
        let send_message_url = &format!("{}/bot{}/setWebhook", self.tg_api_url, self.token);
        let response = self
            .client
            .post(send_message_url)
            .json(&json!({ "url": url }))
            .send()
            .await?;
        Ok(response)
    }
}
