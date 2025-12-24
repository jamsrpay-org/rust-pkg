use crate::{error::SessionError, operation_context::OperationContext};
use redis::RedisClient;

pub struct OperationStore {
    redis: RedisClient,
}

impl OperationStore {
    pub fn new(redis: RedisClient) -> Self {
        Self { redis }
    }

    pub async fn store(&self, context: &OperationContext) -> Result<(), SessionError> {
        let key = format!("operation:{}", context.operation_id);
        let value = serde_json::to_string(context)?;
        let ttl = (context.expires_at - context.created_at).num_seconds() as u64;

        self.redis.set_ex(&key, value, ttl).await?;

        Ok(())
    }

    pub async fn get(&self, operation_id: &str) -> Result<OperationContext, SessionError> {
        let key = format!("operation:{}", operation_id);
        let value: String = self
            .redis
            .get(&key)
            .await?
            .ok_or_else(|| SessionError::TokenExpired)?;

        let context = serde_json::from_str(&value)?;

        Ok(context)
    }

    pub async fn delete(&self, operation_id: &str) -> Result<(), SessionError> {
        let key = format!("operation:{}", operation_id);
        self.redis.delete(&[&key]).await?;

        Ok(())
    }
}
