use crate::errors::AppError;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc, Bson};
use mongodb::Database;
use serde_json::json;
use crate::auth::oauth::models::OAuthTokenRequest;
use crate::auth::oauth::{google, facebook};
use crate::models::user::{Claims, OAuthProvider, User, DEFAULT_AVATAR};
use crate::models::user::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use super::utils::{generate_jwt, hash_password, verify_password};

pub async fn register(
    db: web::Data<Database>,
    data: web::Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    // 验证邮箱是否已存在
    let users_collection = db.collection::<User>("users");
    if users_collection
        .find_one(doc! { "email": &data.email }, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some()
    {
        return Err(AppError::ValidationError("邮箱已注册".to_string()));
    }
    
    // 验证密码强度
    if data.password.len() < 8 {
        return Err(AppError::ValidationError("密码长度必须至少为8位".to_string()));
    }
    
    // 创建新用户
    let hashed_password = hash_password(&data.password)?;
    let new_user = User::new(data.email.clone(), Some(hashed_password));
    
    // 保存用户到数据库
    let insert_result = users_collection
        .insert_one(new_user, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let user_id = insert_result
        .inserted_id
        .as_object_id()
        .expect("插入应该返回ObjectId");
    
    // 生成JWT令牌
    let token = generate_jwt(&user_id.to_string())?;
    
    // 构建响应
    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user_id.to_string(),
            email: data.email.clone(),
            avatar: DEFAULT_AVATAR.to_string(),
        },
    };
    
    Ok(HttpResponse::Created().json(response))
}

pub async fn login(
    db: web::Data<Database>,
    data: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    log::info!("登录成功");
    // 查找用户
    let users_collection = db.collection::<User>("users");
    let user = users_collection
        .find_one(doc! { "email": &data.email }, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::AuthenticationError("用户名或密码错误".to_string()))?;
    
    // 验证密码
    let password_hash = user
        .password_hash
        .as_ref()
        .ok_or_else(|| AppError::AuthenticationError("账户不支持密码登录".to_string()))?;
    
    if !verify_password(&data.password, password_hash)? {
        return Err(AppError::AuthenticationError("用户名或密码错误".to_string()));
    }
    
    // 获取用户ID
    let user_id = user
        .id
        .ok_or_else(|| AppError::InternalError("用户ID不应为空".to_string()))?
        .to_string();
    
    // 生成JWT令牌
    let token = generate_jwt(&user_id)?;
    
    // 构建响应
    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            email: user.email,
            avatar: user.avatar,
        },
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn oauth_login(
    db: web::Data<Database>,
    data: web::Json<OAuthTokenRequest>,
) -> Result<impl Responder, AppError> {
    // 获取用户配置文件信息
    let user_profile = match data.provider.as_str() {
        "google" => google::verify_id_token(&data.token).await?,
        "facebook" => facebook::verify_access_token(&data.token).await?,
        _ => return Err(AppError::ValidationError(format!("不支持的 OAuth 提供商: {}", data.provider))),
    };
    
    // 查找是否已存在该用户（按照 provider + provider_user_id 或 email）
    let users_collection = db.collection::<User>("users");
    
    // 首先按 OAuth 提供商 ID 查找
    let mut user = users_collection
        .find_one(
            doc! {
                "oauth_providers": {
                    "$elemMatch": {
                        "provider": &user_profile.provider,
                        "provider_user_id": &user_profile.provider_user_id
                    }
                }
            },
            None,
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // 如果找不到，则按电子邮件查找
    if user.is_none() {
        user = users_collection
            .find_one(doc! { "email": &user_profile.email }, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }
    
    let now = Utc::now();
    let oauth_provider = OAuthProvider {
        provider: user_profile.provider.clone(),
        provider_user_id: user_profile.provider_user_id.clone(),
        last_login: now,
    };
    
    // 如果用户存在，更新 OAuth 提供商信息
    if let Some(existing_user) = user {
        // 更新 OAuth 提供商列表
        let mut providers = existing_user.oauth_providers.unwrap_or_else(Vec::new);
        
        // 查找是否已存在此提供商
        let provider_index = providers.iter().position(|p| 
            p.provider == oauth_provider.provider && p.provider_user_id == oauth_provider.provider_user_id
        );
        
        if let Some(index) = provider_index {
            // 更新现有提供商的最后登录时间
            providers[index].last_login = now;
        } else {
            // 添加新的提供商
            providers.push(oauth_provider);
        }
        
        // 手动处理 DateTime 转换和序列化问题
        let now_bson = mongodb::bson::DateTime::from_millis(now.timestamp_millis());
        
        let mut providers_bson_array = Vec::new();
        for provider in &providers {
            let provider_doc = bson::doc! {
                "provider": &provider.provider,
                "provider_user_id": &provider.provider_user_id,
                "last_login": mongodb::bson::DateTime::from_millis(provider.last_login.timestamp_millis())
            };
            providers_bson_array.push(Bson::Document(provider_doc));
        }
        
        users_collection
            .update_one(
                doc! { "_id": existing_user.id.unwrap() },
                doc! {
                    "$set": {
                        "oauth_providers": Bson::Array(providers_bson_array),
                        "updated_at": now_bson
                    }
                },
                None,
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let user_id = existing_user.id.unwrap().to_string();
        let token = generate_jwt(&user_id)?;
        
        let response = AuthResponse {
            token,
            user: UserResponse {
                id: user_id,
                email: existing_user.email,
                avatar: existing_user.avatar,
            },
        };
        
        Ok(HttpResponse::Ok().json(response))
    } else {
        // 创建新用户
        let new_user = User {
            id: None,
            email: user_profile.email.clone(),
            password_hash: None,
            oauth_providers: Some(vec![oauth_provider]),
            avatar: DEFAULT_AVATAR.to_string(),
            created_at: now,
            updated_at: now,
        };
        
        let insert_result = users_collection
            .insert_one(new_user, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let user_id = insert_result
            .inserted_id
            .as_object_id()
            .expect("插入应该返回 ObjectId")
            .to_string();
        
        let token = generate_jwt(&user_id)?;
        
        let response = AuthResponse {
            token,
            user: UserResponse {
                id: user_id,
                email: user_profile.email,
                avatar: DEFAULT_AVATAR.to_string(),
            },
        };
        
        Ok(HttpResponse::Created().json(response))
    }
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
    db: web::Data<Database>,
    avatar: web::Json<String>,
) -> Result<HttpResponse, AppError> {
    let claims = get_claims_from_request(&req)?;
    let user_id = ObjectId::parse_str(&claims.sub)
        .map_err(|_| AppError::InvalidId("无效的用户ID".into()))?;

    let now = Utc::now();
    let now_bson = mongodb::bson::DateTime::from_millis(now.timestamp_millis());

    let users_collection = db.collection::<User>("users");
    let result = users_collection
        .update_one(
            doc! { "_id": user_id },
            doc! {
                "$set": {
                    "avatar": avatar.0.clone(), 
                    "updated_at": now_bson
                }
            },
            None,
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if result.modified_count == 0 {
        return Err(AppError::AuthenticationError("用户不存在".to_string()));
    }

    Ok(HttpResponse::Ok().json(json!({
        "avatar": avatar.0
    })))
}

pub async fn get_profile(
    req: HttpRequest,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppError> {
    let claims = get_claims_from_request(&req)?;
    let user_id = ObjectId::parse_str(&claims.sub)
        .map_err(|_| AppError::InvalidId("无效的用户ID".into()))?;

    let users_collection = db.collection::<User>("users");
    let user = users_collection
        .find_one(doc! { "_id": user_id }, None)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::AuthenticationError("用户不存在".to_string()))?;

    Ok(HttpResponse::Ok().json(UserResponse {
        id: user.id.unwrap().to_string(),
        email: user.email,
        avatar: user.avatar,
    }))
}