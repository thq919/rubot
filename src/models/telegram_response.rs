use serde::Deserialize;

use super::Update;

#[derive(Deserialize, Debug, Default)]
pub struct TelegramResponse {
    pub ok: bool,
    pub result: Vec<Update>,
}
