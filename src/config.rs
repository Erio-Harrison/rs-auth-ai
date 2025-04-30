use std::env;
use std::collections::HashMap;
use crate::errors::AppError;
#[derive(Clone, Debug)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub mongodb_uri: String,
    pub database_name: String,
    pub redis_url: String,
    pub ai_providers: AIProviderConfig,
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

    pub fn get_config_value(&self, provider: &str, key: &str) -> Option<&String> {
        self.providers.get(provider).and_then(|config| config.get(key))
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
        
        // 示例：加载通义千问配置
        // 可以根据需要加载其他提供商的配置
        ai_providers.load_from_env("tongyi", &["API_KEY", "API_ENDPOINT"]);
        ai_providers.load_from_env("openai", &["API_KEY", "API_ENDPOINT"]);

        Ok(Config {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| AppError::ConfigError("无效的服务器端口".to_string()))?,
            mongodb_uri: env::var("MONGODB_URI").map_err(|_| {
                AppError::ConfigError("MONGODB_URI 环境变量未设置".to_string())
            })?,
            database_name: env::var("DATABASE_NAME").map_err(|_| {
                AppError::ConfigError("DATABASE_NAME 环境变量未设置".to_string())
            })?,
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6380".to_string()),
            ai_providers,
        })
    }
}