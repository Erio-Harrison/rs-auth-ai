// src/auth/oauth/models.rs
use serde::{Deserialize, Serialize};

// 通用的 OAuth 请求结构
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthTokenRequest {
    pub token: String, 
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthUserProfile {
    pub provider: String,
    pub provider_user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

// Google 特定数据结构
#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
}

// Facebook 特定数据结构
#[derive(Debug, Deserialize)]
pub struct FacebookUserInfo {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "picture")]
    pub picture_data: Option<FacebookPictureData>,
}

#[derive(Debug, Deserialize)]
pub struct FacebookPictureData {
    pub data: Option<FacebookPicture>,
}

#[derive(Debug, Deserialize)]
pub struct FacebookPicture {
    pub url: Option<String>,
}
