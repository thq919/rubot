use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default, Clone)]
pub struct HuggingFaceResponse {
    pub generated_text: String, // This field can vary depending on the response format
}
