use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

pub const DEFAULT_AVATAR: &str = "/default-avatar.png";

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub oauth_provider: Option<String>,
    pub oauth_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub avatar_url: Option<String>,
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
    pub username: String,
    pub avatar: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthUserProfile {
    pub provider: String,
    pub provider_user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

impl User {
    pub async fn create(
        pool: &crate::db::DbPool,
        email: String,
        username: String,
        password_hash: String,
    ) -> Result<Self, crate::errors::AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, username, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(&email)
        .bind(&username)
        .bind(&password_hash)
        .fetch_one(pool)
        .await
        .map_err(|e| crate::errors::AppError::DatabaseError(format!("创建用户失败: {}", e)))?;

        Ok(user)
    }

    pub async fn find_by_email(
        pool: &crate::db::DbPool,
        email: &str,
    ) -> Result<Option<Self>, crate::errors::AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| crate::errors::AppError::DatabaseError(format!("查找用户失败: {}", e)))?;

        Ok(user)
    }

    pub async fn find_by_username(
        pool: &crate::db::DbPool,
        username: &str,
    ) -> Result<Option<Self>, crate::errors::AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(pool)
        .await
        .map_err(|e| crate::errors::AppError::DatabaseError(format!("查找用户失败: {}", e)))?;

        Ok(user)
    }

    pub async fn find_by_id(
        pool: &crate::db::DbPool,
        id: Uuid,
    ) -> Result<Option<Self>, crate::errors::AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| crate::errors::AppError::DatabaseError(format!("查找用户失败: {}", e)))?;

        Ok(user)
    }

    pub async fn update(
        &mut self,
        pool: &crate::db::DbPool,
        update_req: UpdateUserRequest,
    ) -> Result<(), crate::errors::AppError> {
        let mut query_parts = vec!["UPDATE users SET updated_at = NOW()".to_string()];
        let mut param_count = 1;
        let mut params: Vec<&str> = vec![];

        if let Some(ref username) = update_req.username {
            query_parts.push(format!("username = ${}", param_count));
            params.push(username);
            param_count += 1;
        }

        if let Some(ref avatar_url) = update_req.avatar_url {
            query_parts.push(format!("avatar_url = ${}", param_count));
            params.push(avatar_url);
            param_count += 1;
        }

        if params.is_empty() {
            return Ok(());
        }

        let query = format!(
            "{}, {} WHERE id = ${} RETURNING *",
            query_parts[0],
            query_parts[1..].join(", "),
            param_count
        );

        let mut sqlx_query = sqlx::query_as::<_, User>(&query);
        for param in params {
            sqlx_query = sqlx_query.bind(param);
        }
        sqlx_query = sqlx_query.bind(self.id);

        let updated_user = sqlx_query
            .fetch_one(pool)
            .await
            .map_err(|e| crate::errors::AppError::DatabaseError(format!("更新用户失败: {}", e)))?;

        *self = updated_user;
        Ok(())
    }

    pub async fn find_or_create_oauth_user(
        pool: &crate::db::DbPool,
        email: String,
        username: String,
        avatar_url: Option<String>,
        oauth_provider: String,
        oauth_id: String,
    ) -> Result<Self, crate::errors::AppError> {
        if let Some(user) = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE oauth_provider = $1 AND oauth_id = $2"
        )
        .bind(&oauth_provider)
        .bind(&oauth_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| crate::errors::AppError::DatabaseError(format!("查找 OAuth 用户失败: {}", e)))?
        {
            return Ok(user);
        }

        if let Some(existing_user) = Self::find_by_email(pool, &email).await? {
            let updated_user = sqlx::query_as::<_, User>(
                r#"
                UPDATE users 
                SET oauth_provider = $1, oauth_id = $2, updated_at = NOW()
                WHERE id = $3
                RETURNING *
                "#,
            )
            .bind(&oauth_provider)
            .bind(&oauth_id)
            .bind(existing_user.id)
            .fetch_one(pool)
            .await
            .map_err(|e| crate::errors::AppError::DatabaseError(format!("更新用户 OAuth 信息失败: {}", e)))?;

            return Ok(updated_user);
        }

        // 确保用户名唯一
        let final_username = Self::ensure_unique_username(pool, username).await?;

        // 如果都不存在，创建新的 OAuth 用户
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, username, password_hash, avatar_url, oauth_provider, oauth_id, created_at, updated_at)
            VALUES ($1, $2, '', $3, $4, $5, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(&email)
        .bind(&final_username)
        .bind(&avatar_url)
        .bind(&oauth_provider)
        .bind(&oauth_id)
        .fetch_one(pool)
        .await
        .map_err(|e| crate::errors::AppError::DatabaseError(format!("创建 OAuth 用户失败: {}", e)))?;

        Ok(user)
    }

    // 确保用户名唯一
    async fn ensure_unique_username(
        pool: &crate::db::DbPool,
        base_username: String,
    ) -> Result<String, crate::errors::AppError> {
        let mut username = base_username.clone();
        let mut counter = 1;

        while Self::find_by_username(pool, &username).await?.is_some() {
            username = format!("{}_{}", base_username, counter);
            counter += 1;
            
            // 避免无限循环
            if counter > 1000 {
                return Err(crate::errors::AppError::InternalError("无法生成唯一用户名".to_string()));
            }
        }

        Ok(username)
    }
}