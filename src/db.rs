use crate::config::Config;
use crate::errors::AppError;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;

// 使用 SQLx 连接池
pub type DbPool = PgPool;

pub async fn init_db(config: &Config) -> Result<DbPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .min_connections(config.database_min_connections)
        .connect(&config.database_url)
        .await
        .map_err(|e| AppError::DatabaseError(format!("PostgreSQL 连接失败: {}", e)))?;

    let row = sqlx::query("SELECT 1 as test")
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("PostgreSQL 连接验证失败: {}", e)))?;
    
    let test_value: i32 = row.get("test");
    if test_value == 1 {
        log::info!("PostgreSQL 连接成功");
    }

    Ok(pool)
}