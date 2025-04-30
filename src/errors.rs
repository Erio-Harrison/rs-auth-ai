use std::str::Utf8Error;

use actix_multipart::MultipartError;
// src/errors.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("认证失败: {0}")]
    AuthenticationError(String),
    
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    
    #[error("验证错误: {0}")]
    ValidationError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("内部服务器错误: {0}")]
    InternalError(String),

    #[error("无效的帖子")]
    InvalidId(String),

    #[error("缓存出错了")]
    RedisError(String),

    #[error("AI调用出错了")]
    AIServiceError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::AuthenticationError(_) => {
                HttpResponse::Unauthorized().json(json_error_response(&self.to_string()))
            }
            AppError::ValidationError(_) => {
                HttpResponse::BadRequest().json(json_error_response(&self.to_string()))
            }
            AppError::ConfigError(_) => {
                log::error!("配置错误: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("服务器配置错误"))
            }
            AppError::DatabaseError(_) | AppError::InternalError(_) => {
                log::error!("内部错误: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("内部服务器错误"))
            }
            AppError::InvalidId(_) => {
                log::error!("客户端错误: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("无效的帖子，错误"))
            }
            AppError::RedisError(_) => {
                log::error!("缓存错误: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("缓存服务错误"))
            }
            AppError::AIServiceError(_) => {
                log::error!("AI报错: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("AI服务出错"))
            }
        }
    }
}

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        AppError::ValidationError(format!("Multipart error: {}", err))
    }
}

impl From<Utf8Error> for AppError {
    fn from(err: Utf8Error) -> Self {
        AppError::ValidationError(format!("UTF-8 error: {}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::ValidationError(format!("IO error: {}", err))
    }
}

fn json_error_response(message: &str) -> serde_json::Value {
    serde_json::json!({
        "error": message
    })
}