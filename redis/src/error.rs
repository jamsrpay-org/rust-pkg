use redis::RedisError;

#[derive(Debug, thiserror::Error)]
pub enum RedisClientError {
    #[error("Redis connection error: {0}")]
    ConnectionError(#[from] RedisError),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Key not found: {0}")]
    KeyNotFound(String),
}
