use serde::Deserialize;

use super::Message;

#[derive(Deserialize, Debug, Default)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
}
