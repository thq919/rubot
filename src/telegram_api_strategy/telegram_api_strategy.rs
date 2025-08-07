use actix_client::telegram_webhook;
use actix_web::{
    rt::time::sleep,
    web::{post, Data},
    App, HttpServer,
};
use log::error;
use std::sync::Arc;
use std::time::Duration;

use crate::{
    actix_client, env_config::EnvConfig, models::{chat, message, TelegramResponse}, response_former::ResponseFormer,
    telegram_api::TelegramApi, types::Result,
};

pub struct TelegramApiStrategy {
    response_former: Arc<ResponseFormer>,
    api: Arc<TelegramApi>,
    mode: BotMode,
    config: Arc<EnvConfig>,
}

pub enum BotMode {
    Webhook,
    Polling,
}

impl TelegramApiStrategy {
    pub fn new(
        mode: BotMode,
        api: Arc<TelegramApi>,
        config: Arc<EnvConfig>,
        response_former: Arc<ResponseFormer>,
    ) -> TelegramApiStrategy {
        TelegramApiStrategy {
            mode: mode,
            api: api,
            config: config,
            response_former: response_former,
        }
    }

    pub async fn start(&self) -> Result<()> {
        return match self.mode {
            BotMode::Webhook => {
                let _ = self.api.set_webhook(&self.config.webhook_url).await;
                self._webhook().await
            }
            BotMode::Polling => {
                let _ = self.api.delete_webhook().await;
                self._polling().await
            }
        };
    }

    async fn _polling(&self) -> Result<()> {
        let mut last_update_id: i64 = 0;
        loop {
            sleep(Duration::from_secs(3)).await;

            let Ok(response) = self.api.get_updates(last_update_id).await else {
                error!("Ошибка при получении обновлений.");
                continue;
            };
        
            let Ok(response) = response.json::<TelegramResponse>().await else {
                eprintln!("Ошибка при парсинге JSON");                
                continue;
            };

            for msg in response.result {
                let Some(message) = msg.message else {continue;};
                let Some(chat)= message.chat else {continue;};
                let Some(id) = chat.id else {continue;};
                if id == 0 {continue};
        
                
                let text = message.text.unwrap_or_default();
                let reply_text = self.response_former.form(text).await;

                let _ = self.api.send_message(&id, &reply_text).await;
                last_update_id = msg.update_id.unwrap_or_default() + 1;
            }
        }
    }

    async fn _run_server(&self) -> std::result::Result<(), std::io::Error> {
        let api_clone = self.api.clone();
        let config_clone = self.config.clone();
        let response_former_clone = self.response_former.clone();
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(api_clone.clone()))
                .app_data(Data::new(config_clone.clone()))
                .app_data(Data::new(response_former_clone.clone()))
                .route("/telegram_webhook", post().to(telegram_webhook))
        })
        .bind("0.0.0.0:8080")?
        .run()
        .await
    }

    async fn _webhook(&self) -> Result<()> {
        let r: std::result::Result<(), std::io::Error> =
            TelegramApiStrategy::_run_server(&self).await;
        match r {
            Ok(e) => Ok(e),
            Err(e) => Err(e.into()),
        }
    }
}
