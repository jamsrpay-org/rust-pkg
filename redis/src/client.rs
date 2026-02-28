use crate::error::RedisClientError;
use redis::{
    AsyncCommands as _, Client, FromRedisValue, ToRedisArgs,
    aio::{ConnectionManager, ConnectionManagerConfig},
};
use std::{collections::HashMap, fmt::Display};
pub struct RedisClient {
    manager: ConnectionManager,
    client: Client,
}

impl RedisClient {
    pub async fn new(redis_url: &str) -> Result<Self, RedisClientError> {
        let client = Client::open(redis_url)?;
        let manager = ConnectionManager::new(client.clone()).await?;

        Ok(Self { manager, client })
    }

    pub async fn new_with_config(
        redis_url: &str,
        config: ConnectionManagerConfig,
    ) -> Result<Self, RedisClientError> {
        let client = Client::open(redis_url)?;
        let manager = ConnectionManager::new_with_config(client.clone(), config).await?;

        Ok(Self { manager, client })
    }

    pub async fn set<K, V>(&self, key: K, value: V) -> Result<(), RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();

        conn.set::<K, V, ()>(key, value).await?;
        Ok(())
    }

    pub async fn set_ex<K, V>(&self, key: K, value: V, ttl: u64) -> Result<(), RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        conn.set_ex::<K, V, ()>(key, value, ttl).await?;

        Ok(())
    }

    pub async fn get<K, V>(&self, key: K) -> Result<Option<V>, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        V: FromRedisValue + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let value: Option<V> = conn.get::<K, Option<V>>(key).await?;

        Ok(value)
    }

    pub async fn get_required<K, V>(&self, key: K) -> Result<V, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync + Display,
        V: FromRedisValue + Send + Sync,
    {
        self.get(&key)
            .await?
            .ok_or_else(|| RedisClientError::KeyNotFound(key.to_string()))
    }

    pub async fn delete<K>(&self, keys: &[K]) -> Result<u64, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let deleted: u64 = conn.del(keys).await?;
        Ok(deleted)
    }

    pub async fn exists<K>(&self, key: K) -> Result<bool, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let exists: bool = conn.exists(&key).await?;
        Ok(exists)
    }

    pub async fn expiry<K>(&self, key: K, expiry_secs: u64) -> Result<bool, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let result: bool = conn.expire(key, expiry_secs as i64).await?;
        Ok(result)
    }

    pub async fn ttl<K>(&self, key: K) -> Result<i64, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let result: i64 = conn.ttl(key).await?;
        Ok(result)
    }

    pub async fn increment<K>(&self, key: K, delta: i64) -> Result<i64, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let new_value: i64 = conn.incr(key, delta).await?;
        Ok(new_value)
    }

    pub async fn hset_multiple<K, F, V>(
        &self,
        key: K,
        fields: &[(F, V)],
    ) -> Result<(), RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        F: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        conn.hset_multiple::<K, F, V, ()>(key, fields).await?;

        Ok(())
    }

    pub async fn hget<K, F>(&self, key: K, field: F) -> Result<Option<String>, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        F: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let value = conn.hget(key, field).await?;

        Ok(value)
    }

    pub async fn hgetall<K>(&self, key: K) -> Result<HashMap<String, String>, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync + Clone,
    {
        let mut conn = self.manager.clone();
        let hash = conn.hgetall(key).await?;

        Ok(hash)
    }

    pub async fn hincrby<K, F>(&self, key: K, field: F, delta: i64) -> Result<i64, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        F: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let new_value: i64 = conn.hincr(key, field, delta).await?;
        Ok(new_value)
    }

    pub async fn pipeline_set_multiple<K, V>(
        &self,
        items: &[(K, V)],
    ) -> Result<(), RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let mut pipe = redis::pipe();

        for (key, value) in items {
            pipe.set(key, value).ignore();
        }

        pipe.query_async::<()>(&mut conn).await?;

        Ok(())
    }

    pub async fn ping(&self) -> Result<(), RedisClientError> {
        let mut conn = self.manager.clone();
        conn.ping::<()>().await?;

        Ok(())
    }

    pub async fn sadd<K, V>(&self, key: K, value: V) -> Result<bool, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let result: bool = conn.sadd(key, value).await?;
        Ok(result)
    }

    pub async fn srem<K, V>(&self, key: K, value: V) -> Result<bool, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let result: bool = conn.srem(key, value).await?;
        Ok(result)
    }

    pub async fn smembers<K>(&self, key: K) -> Result<Vec<String>, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let result: Vec<String> = conn.smembers(key).await?;
        Ok(result)
    }

    pub async fn sismember<K, V>(&self, key: K, value: V) -> Result<bool, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let result: bool = conn.sismember(key, value).await?;
        Ok(result)
    }

    pub async fn publish<K, V>(&self, channel: K, message: V) -> Result<u64, RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
        V: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        let result: u64 = conn.publish(channel, message).await?;
        Ok(result)
    }

    pub async fn subscribe<K, V>(&self, channel: K) -> Result<(), RedisClientError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.manager.clone();
        conn.subscribe(channel).await?;
        Ok(())
    }

    pub async fn get_pubsub(&self) -> Result<redis::aio::PubSub, RedisClientError> {
        let pubsub = self.client.get_async_pubsub().await?;
        Ok(pubsub)
    }
}
