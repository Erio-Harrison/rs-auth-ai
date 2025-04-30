// src/auth/oauth/google.rs
use crate::errors::AppError;
use crate::auth::oauth::models::{GoogleUserInfo, OAuthUserProfile};
use reqwest::Client;

pub async fn verify_id_token(token: &str) -> Result<OAuthUserProfile, AppError> {
    // Google 提供了一个 tokeninfo 端点用于验证 ID 令牌
    let url = format!(
        "https://oauth2.googleapis.com/tokeninfo?id_token={}",
        token
    );
    
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::AuthenticationError(format!("Google API 请求失败: {}", e)))?;
    
    // 检查状态码
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::AuthenticationError(format!("Google 令牌验证失败: {}", error_text)));
    }
    
    // 解析用户信息
    let user_info: GoogleUserInfo = response
        .json()
        .await
        .map_err(|e| AppError::AuthenticationError(format!("解析 Google 用户信息失败: {}", e)))?;
    
    // 验证电子邮件（可选）
    if let Some(verified) = user_info.email_verified {
        if !verified {
            return Err(AppError::AuthenticationError("Google 电子邮件未经验证".to_string()));
        }
    }
    
    // 转换为通用的用户配置文件格式
    Ok(OAuthUserProfile {
        provider: "google".to_string(),
        provider_user_id: user_info.sub,
        email: user_info.email,
        name: user_info.name,
        picture: user_info.picture,
    })
}