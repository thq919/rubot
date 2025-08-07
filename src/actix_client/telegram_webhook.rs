use crate::{models::Update, response_former::ResponseFormer, telegram_api::TelegramApi};
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use log::error;
use std::sync::Arc;

pub async fn telegram_webhook(
    update: Json<Update>,
    api: Data<Arc<TelegramApi>>,
    response_former: Data<Arc<ResponseFormer>>,
) -> impl Responder {
    if update.message.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    let update = update.into_inner();
    let message = update.message.unwrap_or_default();
    let text: String = message.text.unwrap_or_default();

    let chat_id = message.chat.unwrap_or_default().id.unwrap_or_default();

    let response = api
        .send_message(&chat_id, &response_former.form(text).await)
        .await;

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
