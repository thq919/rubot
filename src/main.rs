use actix_client::telegram_webhook;
use actix_web::{
    web::{post, Data},
    App, HttpServer,
};
use env_config::EnvConfig;
use telegram_api::TelegramApi;

mod actix_client;
mod env_config;
mod models;
mod telegram_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = EnvConfig::new();
    let api = TelegramApi::new(&config);

    if let Err(e) = api.set_webhook(&config.webhook_url).await {
        eprintln!("Ошибка при установке webhook: {}", e);
    }

    HttpServer::new(move ||  {
        App::new()
            .app_data(Data::new(api.clone()))
            .app_data(Data::new(config.clone()))
            .route("/telegram_webhook", post().to(telegram_webhook))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
