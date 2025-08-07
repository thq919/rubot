#![allow(dead_code, unused_imports)]

use env_config::EnvConfig;
use env_logger;
use huggin_face_api::HuggingFaceAPI;
use reqwest::Client;
use response_former::ResponseFormer;
use telegram_api::TelegramApi;
use telegram_api_strategy::{BotMode, TelegramApiStrategy};
use types::Result;

mod actix_client;
mod env_config;
mod huggin_face_api;
mod models;
mod response_former;
mod telegram_api;
mod telegram_api_strategy;
mod types;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    let arc_config = Arc::new(EnvConfig::new());
    let arc_client = Arc::new(Client::new());
    let arc_openai_api = Arc::new(HuggingFaceAPI::new(arc_config.clone(), arc_client.clone()));
    let telegram_api = TelegramApi::new(arc_config.clone(), arc_client.clone());
    let arc_telegram_api = Arc::new(telegram_api);
    let arc_response_former = Arc::new(ResponseFormer::new(arc_openai_api));

    TelegramApiStrategy::new(
        BotMode::Polling,
        arc_telegram_api,
        arc_config,
        arc_response_former,
    )
    .start()
    .await
}
