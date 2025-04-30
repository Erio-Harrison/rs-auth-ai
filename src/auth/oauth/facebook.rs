// src/auth/oauth/facebook.rs
use crate::errors::AppError;
use crate::auth::oauth::models::{FacebookUserInfo, OAuthUserProfile};
use reqwest::Client;

pub async fn verify_access_token(token: &str) -> Result<OAuthUserProfile, AppError> {
    let url = format!(
        "https://graph.facebook.com/me?fields=id,name,email,picture&access_token={}",
        token
    );
    
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::AuthenticationError(format!("Facebook API 请求失败: {}", e)))?;
    
    // 检查状态码
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::AuthenticationError(format!("Facebook 令牌验证失败: {}", error_text)));
    }
    
    // 解析用户信息
    let user_info: FacebookUserInfo = response
        .json()
        .await
        .map_err(|e| AppError::AuthenticationError(format!("解析 Facebook 用户信息失败: {}", e)))?;
    
    // 确保有电子邮件
    let email = user_info.email.ok_or_else(|| {
        AppError::AuthenticationError("Facebook 帐户没有关联电子邮件".to_string())
    })?;
    
    // 获取头像 URL (如果有)
    let picture = user_info
        .picture_data
        .and_then(|data| data.data)
        .and_then(|pic| pic.url);
    
    // 转换为通用的用户配置文件格式
    Ok(OAuthUserProfile {
        provider: "facebook".to_string(),
        provider_user_id: user_info.id,
        email,
        name: user_info.name,
        picture,
    })
}
