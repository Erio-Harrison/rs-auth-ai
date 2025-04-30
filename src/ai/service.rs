use std::sync::Arc;

use crate::errors::AppError;
use crate::models::ai::{AIRequest, AIResponse};
use crate::config::Config;
use async_trait::async_trait;

use super::providers::{self, Provider};

#[async_trait]
pub trait AIService: Send + Sync {
    async fn analyze(&self, request: AIRequest) -> Result<AIResponse, AppError>;
}

#[derive(Clone)]
pub struct AIServiceImpl {
    config: Arc<Config>
}

impl AIServiceImpl {
    pub fn new(config: Config) -> Self {
        Self { config: config.into() }
    }
}

#[async_trait]
impl AIService for AIServiceImpl {
    async fn analyze(&self, request: AIRequest) -> Result<AIResponse, AppError> {
        let provider = providers::tongyi::TongyiProvider::new(&self.config)?;
        provider.analyze(request).await
    }
}