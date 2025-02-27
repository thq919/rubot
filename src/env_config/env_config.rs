use std::env;

#[derive(Clone)]
pub struct EnvConfig {
    pub token: String,
    pub webhook_url: String,
    pub tg_api_url: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        EnvConfig {
            token: env::var("TOKEN").expect("TOKEN must be set"),
            webhook_url: env::var("HOST").expect("HOST must be set"),
            tg_api_url: env::var("TELEGRAM_API_URL").expect("TELEGRAM_API_URL must be set"),
        }
    }
}
