use crate::env_config::EnvConfig;
use crate::models::huggin_face_response::{self, HuggingFaceResponse};
use crate::types::Result;
use log::{error, info};
use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::sync::Arc;

pub struct HuggingFaceAPI {
    pub token: String,
    pub huggingface_api_url: String,
    pub client: Arc<Client>,
}

impl HuggingFaceAPI {
    pub fn new(env_h: Arc<EnvConfig>, client: Arc<Client>) -> Self {
        HuggingFaceAPI {
            token: env_h.huggingface_token.to_string(),
            huggingface_api_url: env_h.huggingface_url.to_string(),
            client,
        }
    }

    pub async fn send_message(&self, prompt: &str) -> Result<HuggingFaceResponse> {
        let url = format!(
            "{}/models/TrollfaceMod/Trollface-GPT",
            &self.huggingface_api_url
        );

        info!("Sending request to Hugging Face: {}", url);
        info!("Request text: {}", prompt);

        let r: std::result::Result<reqwest::Response, reqwest::Error> = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&json!({ "inputs": prompt }))
            .send() 
            .await;

        let r: reqwest::Response = r?;
        let r: String = r.text().await?;
        let r: HuggingFaceResponse = serde_json::from_str(&r)?;
        Ok(r)
    }
}
