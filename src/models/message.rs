use serde::Deserialize;

use super::chat::Chat;

#[derive(Deserialize, Debug, Default)]
pub struct Message {
    pub message_id: i64,
    pub text: Option<String>,
    pub chat: Chat,
}
