use crate::{models::Update, telegram_api::TelegramApi};
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use log::error;

pub async fn telegram_webhook(update: Json<Update>, api: Data<TelegramApi>) -> impl Responder {
    if update.message.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    let update = update.into_inner();
    let message = update.message.unwrap_or_default();
    let text: String = message.text.unwrap_or_default();

    let reply_text = match text.as_str() {
        "/start" => "Привет! Я сервер на Rust".to_string(),
        _ => format!("Вы написали: {}", text),
    };

    let response = api.send_message(&message.chat.id, &reply_text).await;

    match response {
        Ok(res) => {
            if let Err(e) = res.error_for_status_ref() {
                error!("Telegram API error: {}", e);
            }
        }
        Err(e) => error!("Failed to send message: {}", e),
    }
    HttpResponse::Ok().finish()
}
