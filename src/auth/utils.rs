// src/auth/utils.rs
use crate::errors::AppError;
use crate::models::user::Claims;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::InternalError(format!("密码哈希失败: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::InternalError(format!("密码解析失败: {}", e)))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_jwt(user_id: &str) -> Result<String, AppError> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "604800".to_string()) 
        .parse::<i64>()
        .unwrap_or(604800);
    
    let now = Utc::now();
    let expiry = now + Duration::seconds(expiration);
    
    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp() as usize,
        exp: expiry.timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalError(format!("JWT 生成失败: {}", e)))
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default()
    )
    .map(|token_data| token_data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => 
            AppError::AuthenticationError("Token已过期".to_string()),
        _ => AppError::AuthenticationError("无效的Token".to_string())
    })
}