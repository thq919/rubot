use std::env;

#[derive(Clone)]
pub struct EnvConfig {
    pub tg_token: String,
    pub openai_token: String,
    pub webhook_url: String,
    pub tg_api_url: String,
    pub openai_api_url: String,
    pub huggingface_token: String,
    pub huggingface_url: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        EnvConfig {
            tg_token: env::var("TG_TOKEN").expect("TOKEN must be set"),
            openai_token: env::var("OPENAI_TOKEN").expect("TOKEN must be set"),
            webhook_url: env::var("HOST").expect("HOST must be set"),
            tg_api_url: env::var("TELEGRAM_API_URL").expect("TELEGRAM_API_URL must be set"),
            openai_api_url: env::var("OPENAI_API_URL").expect("TELEGRAM_API_URL must be set"),
            huggingface_token: env::var("HUGGIN_FACE_TOKEN").expect("TELEGRAM_API_URL must be set"),
            huggingface_url: env::var("HUGGIN_FACE_URL").expect("TELEGRAM_API_URL must be set"),
        }
    }
}
