use crate::config::Config;
use crate::errors::AppError;
use mongodb::bson::doc;
use mongodb::{Client, Database};
use mongodb::options::ClientOptions;

pub async fn init_db(config: &Config) -> Result<Database, AppError> {
    let client_options = ClientOptions::parse(&config.mongodb_uri)
        .await
        .map_err(|e| AppError::DatabaseError(format!("MongoDB 连接配置错误: {}", e)))?;
    
    let client = Client::with_options(client_options)
        .map_err(|e| AppError::DatabaseError(format!("MongoDB 客户端创建失败: {}", e)))?;
    
    // 验证连接是否成功
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
        .map_err(|e| AppError::DatabaseError(format!("MongoDB 连接验证失败: {}", e)))?;
    
    log::info!("MongoDB 连接成功");
    Ok(client.database(&config.database_name))
}