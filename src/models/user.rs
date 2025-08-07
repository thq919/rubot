use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct User {
    pub id: Option<i64>,
    pub is_bot: Option<bool>,
    pub first_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
}
