use crate::errors::AppError;

#[derive(Clone)]
pub struct RedisService {
    client: redis::Client,
}

impl RedisService {
    pub fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| AppError::RedisError(e.to_string()))?;
        Ok(Self { client })
    }
}