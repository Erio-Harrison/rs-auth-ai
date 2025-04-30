use async_trait::async_trait;
use crate::errors::AppError;
use crate::models::ai::{AIRequest, AIResponse};

pub enum ImageFormat {
    Binary(Vec<u8>),
    Base64(String),
    Url(String),
}

#[async_trait]
pub trait Provider: Send + Sync {
    fn process_image(&self, image_data: Vec<u8>) -> Result<ImageFormat, AppError>;
    fn get_endpoint(&self, is_multimodal: bool) -> String;
    async fn analyze(&self, request: AIRequest) -> Result<AIResponse, AppError>;
}

pub mod tongyi;