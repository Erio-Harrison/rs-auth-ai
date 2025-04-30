use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const DEFAULT_AVATAR: &str = "/default-avatar.png";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_providers: Option<Vec<OAuthProvider>>,
    pub avatar: String, 
    #[serde(with = "crate::models::datetime_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::models::datetime_format")]

    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthProvider {
    pub provider: String,
    pub provider_user_id: String,
    #[serde(with = "crate::models::datetime_format")]
    pub last_login: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub avatar: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl User {
    pub fn new(email: String, password_hash: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            email,
            password_hash,
            oauth_providers: None,
            avatar: DEFAULT_AVATAR.to_string(), 
            created_at: now,
            updated_at: now,
        }
    }
}