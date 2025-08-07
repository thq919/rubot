use actix_web::http::Error;
use log::info;

use crate::{
    huggin_face_api::HuggingFaceAPI,
    models::huggin_face_response::{self, HuggingFaceResponse},
    types::Result,
};

use std::sync::Arc;

#[derive(Clone)]
pub struct ResponseFormer {
    huggin_face_api: Arc<HuggingFaceAPI>,
}

impl ResponseFormer {
    pub fn new(huggin_face_api: Arc<HuggingFaceAPI>) -> ResponseFormer {
        return ResponseFormer {
            huggin_face_api: huggin_face_api,
        };
    }

    pub async fn form(&self, msg: String) -> String {
        match msg.as_str() {
            "/start" => "Привет! Я сервер на Rust".to_string(),
            _ => {
                let r: Result<HuggingFaceResponse> = self.huggin_face_api.send_message(&msg).await;
                match r {
                    Ok(r) => r.generated_text,
                    Err(_) => "вышла ошибочка".to_string(),
                }
            }
        }
    }
}
