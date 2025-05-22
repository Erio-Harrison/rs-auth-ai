use std::env;
use std::collections::HashMap;
use crate::errors::AppError;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub ai_providers: AIProviderConfig,
    pub database_max_connections: u32,
    pub database_min_connections: u32,
}

#[derive(Clone, Debug, Default)]
pub struct AIProviderConfig {
    providers: HashMap<String, HashMap<String, String>>,
}

impl AIProviderConfig {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn get_provider_config(&self, provider: &str) -> Option<&HashMap<String, String>> {
        self.providers.get(provider)
    }

    pub fn load_from_env(&mut self, provider: &str, config_keys: &[&str]) {
        let mut config = HashMap::new();
        for key in config_keys {
            let env_key = format!("AI_{}_{}",provider.to_uppercase(), key.to_uppercase());
            if let Ok(value) = env::var(&env_key) {
                config.insert(key.to_string(), value);
            }
        }
        if !config.is_empty() {
            self.providers.insert(provider.to_string(), config);
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let mut ai_providers = AIProviderConfig::new();
        
        ai_providers.load_from_env("tongyi", &["API_KEY", "API_ENDPOINT"]);
        ai_providers.load_from_env("openai", &["API_KEY", "API_ENDPOINT"]);

        Ok(Config {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| AppError::ConfigError("无效的服务器端口".to_string()))?,
            
            database_url: env::var("DATABASE_URL").map_err(|_| {
                AppError::ConfigError("DATABASE_URL 环境变量未设置".to_string())
            })?,
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .map_err(|_| AppError::ConfigError("无效的数据库最大连接数".to_string()))?,
            database_min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .map_err(|_| AppError::ConfigError("无效的数据库最小连接数".to_string()))?,
            
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            ai_providers,
        })
    }
}