use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Chat {
    pub id: i64,
}
