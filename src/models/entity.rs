use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Entity {
    pub offset: i64,
    pub length: i64,
    pub r#type: String,
}
