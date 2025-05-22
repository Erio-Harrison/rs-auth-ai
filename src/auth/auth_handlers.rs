use crate::errors::AppError;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use serde_json::json;
use crate::auth::oauth::models::OAuthTokenRequest;
use crate::auth::oauth::{google, facebook};
use crate::models::user::{Claims, User, DEFAULT_AVATAR};
use crate::models::user::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use super::utils::{generate_jwt, hash_password, verify_password};

pub async fn register(
    db: web::Data<crate::db::DbPool>,
    data: web::Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    if User::find_by_email(&db, &data.email).await?.is_some() {
        return Err(AppError::ValidationError("邮箱已注册".to_string()));
    }
    
    if User::find_by_username(&db, &data.username).await?.is_some() {
        return Err(AppError::ValidationError("用户名已存在".to_string()));
    }
    
    if data.password.len() < 8 {
        return Err(AppError::ValidationError("密码长度必须至少为8位".to_string()));
    }
    
    let hashed_password = hash_password(&data.password)?;
    let user = User::create(&db, data.email.clone(), data.username.clone(), hashed_password).await?;
    
    let token = generate_jwt(&user.id.to_string())?;
    
    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            username: user.username,
            avatar: user.avatar_url.unwrap_or_else(|| DEFAULT_AVATAR.to_string()),
        },
    };
    
    Ok(HttpResponse::Created().json(response))
}

pub async fn login(
    db: web::Data<crate::db::DbPool>,
    data: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let user = User::find_by_email(&db, &data.email).await?
        .ok_or_else(|| AppError::AuthenticationError("用户名或密码错误".to_string()))?;

    if user.password_hash.is_empty() {
        return Err(AppError::AuthenticationError("账户不支持密码登录".to_string()));
    }

    if !verify_password(&data.password, &user.password_hash)? {
        return Err(AppError::AuthenticationError("用户名或密码错误".to_string()));
    }

    let token = generate_jwt(&user.id.to_string())?;

    let user_email = user.email.clone();
    let user_id = user.id.to_string();
    let user_username = user.username.clone();
    let user_avatar = user.avatar_url.clone();

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            email: user_email.clone(),
            username: user_username,
            avatar: user_avatar.unwrap_or_else(|| DEFAULT_AVATAR.to_string()),
        },
    };

    log::info!("用户登录成功: {}", user_email);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn oauth_login(
    db: web::Data<crate::db::DbPool>,
    data: web::Json<OAuthTokenRequest>,
) -> Result<impl Responder, AppError> {
    let user_profile = match data.provider.as_str() {
        "google" => google::verify_id_token(&data.token).await?,
        "facebook" => facebook::verify_access_token(&data.token).await?, 
        _ => return Err(AppError::ValidationError(format!("不支持的 OAuth 提供商: {}", data.provider))),
    };

    let user = User::find_or_create_oauth_user(
        &db,
        user_profile.email.clone(),
        user_profile.name.unwrap_or_else(|| user_profile.email.split('@').next().unwrap_or("user").to_string()),
        user_profile.picture,
        user_profile.provider.clone(),
        user_profile.provider_user_id.clone(),
    ).await?;

    let token = generate_jwt(&user.id.to_string())?;

    let user_email = user.email.clone();
    let user_id = user.id.to_string();
    let user_username = user.username.clone();
    let user_avatar = user.avatar_url.clone();
    let provider_name = data.provider.clone();

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            email: user_email.clone(),
            username: user_username,
            avatar: user_avatar.unwrap_or_else(|| DEFAULT_AVATAR.to_string()),
        },
    };

    log::info!("OAuth 登录成功: {} ({})", user_email, provider_name);
    Ok(HttpResponse::Ok().json(response))
}

pub fn get_claims_from_request(req: &HttpRequest) -> Result<Claims, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::AuthenticationError("缺少认证Token".to_string()))?;

    crate::auth::utils::verify_jwt(token)
}

pub async fn update_avatar(
    req: HttpRequest,
    db: web::Data<crate::db::DbPool>,
    avatar: web::Json<String>,
) -> Result<HttpResponse, AppError> {
    let claims = get_claims_from_request(&req)?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::InvalidId("无效的用户ID".into()))?;

    let mut user = User::find_by_id(&db, user_id).await?
        .ok_or_else(|| AppError::AuthenticationError("用户不存在".to_string()))?;

    let update_req = crate::models::user::UpdateUserRequest {
        username: None,
        avatar_url: Some(avatar.0.clone()),
    };
    
    user.update(&db, update_req).await?;

    Ok(HttpResponse::Ok().json(json!({
        "avatar": avatar.0
    })))
}

pub async fn get_profile(
    req: HttpRequest,
    db: web::Data<crate::db::DbPool>,
) -> Result<HttpResponse, AppError> {
    let claims = get_claims_from_request(&req)?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::InvalidId("无效的用户ID".into()))?;

    let user = User::find_by_id(&db, user_id).await?
        .ok_or_else(|| AppError::AuthenticationError("用户不存在".to_string()))?;

    Ok(HttpResponse::Ok().json(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        username: user.username,
        avatar: user.avatar_url.unwrap_or_else(|| DEFAULT_AVATAR.to_string()),
    }))
}