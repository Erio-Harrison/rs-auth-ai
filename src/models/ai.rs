use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum AIInput {
    Text(String),
    Image(Vec<u8>),
    ImageWithText {
        image: Vec<u8>,
        text: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIRequest {
    pub input: AIInput,
    pub model: Option<String>,
    pub prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIResponse {
    pub content: String,
    pub confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_response: Option<serde_json::Value>,
}