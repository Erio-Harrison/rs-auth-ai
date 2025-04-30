use super::{Provider, ImageFormat};
use crate::config::Config;
use crate::errors::AppError;
use crate::models::ai::{AIRequest, AIResponse, AIInput};
use reqwest::Client;
use async_trait::async_trait;
use serde_json::json;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub struct TongyiProvider {
    client: Client,
    api_key: String,
    text_endpoint: String,
    multimodal_endpoint: String,
}

impl TongyiProvider {
    pub fn new(config: &Config) -> Result<Self, AppError> {
        let provider_config = config.ai_providers.get_provider_config("tongyi")
            .ok_or_else(|| AppError::ConfigError("Tongyi provider not configured".to_string()))?;

        let api_key = provider_config.get("API_KEY")
            .ok_or_else(|| AppError::ConfigError("Tongyi API_KEY not configured".to_string()))?
            .clone();

        Ok(Self {
            client: Client::new(),
            api_key,
            text_endpoint: "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation".to_string(),
            multimodal_endpoint: "https://dashscope.aliyuncs.com/api/v1/services/aigc/multimodal-generation/generation".to_string(),
        })
    }

    fn build_text_payload(&self, text: &str, model: &str) -> serde_json::Value {
        json!({
            "model": model,
            "input": {
                "messages": [
                    {
                        "role": "user",
                        "content": text
                    }
                ]
            }
        })
    }

    fn build_image_payload(&self, image: String, text: String, model: &str) -> serde_json::Value {
        json!({
            "model": model,
            "input": {
                "messages": [
                    {
                        "role": "user",
                        "content": [
                            { "image": image },
                            { "text": text }
                        ]
                    }
                ]
            }
        })
    }
}

#[async_trait]
impl Provider for TongyiProvider {
    fn process_image(&self, image_data: Vec<u8>) -> Result<ImageFormat, AppError> {
        log::debug!("Processing image with size: {} bytes", image_data.len());
        if image_data.len() < 100 {
            return Err(AppError::ValidationError("Invalid image data".to_string()));
        }
        let base64 = STANDARD.encode(image_data);
        Ok(ImageFormat::Base64(format!("data:image/jpeg;base64,{}", base64)))
    }

    fn get_endpoint(&self, is_multimodal: bool) -> String {
        if is_multimodal {
            self.multimodal_endpoint.clone()
        } else {
            self.text_endpoint.clone()
        }
    }

    async fn analyze(&self, request: AIRequest) -> Result<AIResponse, AppError> {
        // 保存输入类型
        let is_text = matches!(&request.input, AIInput::Text(_));

        let (_model, payload, endpoint) = match request.input {
            AIInput::Text(text) => {
                let model = request.model.unwrap_or_else(|| "qwen-turbo".to_string());
                let payload = self.build_text_payload(&text, &model);
                (model, payload, self.get_endpoint(false))
            },
            AIInput::Image(image_data) => {
                let model = String::from("qwen-vl-max");
                let processed_image = self.process_image(image_data)?;
                let image_content = match processed_image {
                    ImageFormat::Base64(base64) => base64,
                    _ => return Err(AppError::AIServiceError("Unsupported image format".to_string())),
                };
                let text = request.prompt.unwrap_or_else(|| "请分析这张图片".to_string());
                let payload = self.build_image_payload(image_content, text, &model);
                (model, payload, self.get_endpoint(true))
            },
            AIInput::ImageWithText { image, text } => {
                let model = String::from("qwen-vl-max");
                let processed_image = self.process_image(image)?;
                let image_content = match processed_image {
                    ImageFormat::Base64(base64) => base64,
                    _ => return Err(AppError::AIServiceError("Unsupported image format".to_string())),
                };
                let prompt = request.prompt.unwrap_or_else(|| text);
                let payload = self.build_image_payload(image_content, prompt, &model);
                (model, payload, self.get_endpoint(true))
            }
        };

        log::debug!("Sending request to Tongyi API: {:?}", payload);

        let response = self.client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::AIServiceError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(AppError::AIServiceError(format!("API error: {}", error)));
        }

        let response_data = response.json::<serde_json::Value>().await
            .map_err(|e| AppError::AIServiceError(format!("Parse response failed: {}", e)))?;

        log::debug!("Tongyi API response: {:?}", response_data);

        let content = if is_text {
            response_data
                .get("output")
                .and_then(|output| output.get("text"))
                .and_then(|text| text.as_str())
                .ok_or_else(|| AppError::AIServiceError("Invalid response format".to_string()))?
                .to_string()
        } else {
            response_data
                .get("output")
                .and_then(|output| output.get("choices"))
                .and_then(|choices| choices.get(0))
                .and_then(|choice| choice.get("message"))
                .and_then(|message| message.get("content"))
                .and_then(|content| content.get(0))
                .and_then(|first| first.get("text"))
                .and_then(|text| text.as_str())
                .ok_or_else(|| AppError::AIServiceError("Invalid response format".to_string()))?
                .to_string()
        };

        Ok(AIResponse {
            content,
            confidence: None,
            raw_response: Some(response_data),
        })
    }
}