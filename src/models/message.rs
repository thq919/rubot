use serde::Deserialize;

use super::chat::Chat;
use super::{Entity, User};

#[derive(Deserialize, Debug, Default)]
pub struct Message {
    pub message_id: Option<i64>,
    pub from: Option<User>,
    pub chat: Option<Chat>,
    pub date: Option<i64>,
    pub text: Option<String>,
    pub entities: Option<Vec<Entity>>,
}
