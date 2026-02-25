use std::sync::Arc;

use crate::{
    continuation_token::ContinuationToken, error::SessionError,
    operation_context::OperationContext, operation_store::OperationStore,
};
use redis::RedisClient;
use serde::{Serialize, de::DeserializeOwned};

pub struct TokenOrchestrator {
    operation_store: OperationStore,
    secret_key: String,
}

impl TokenOrchestrator {
    pub fn new(redis: Arc<RedisClient>, secret_key: String) -> Self {
        Self {
            operation_store: OperationStore::new(redis),
            secret_key,
        }
    }

    pub async fn initiate_operation<T: Serialize>(
        &self,
        operation_type: String,
        data: T,
    ) -> Result<(String, OperationContext), SessionError> {
        let data = serde_json::to_value(data)?;
        let context = OperationContext::new(operation_type, data);
        self.operation_store.store(&context).await?;

        let token = ContinuationToken::new(
            context.operation_id.clone(),
            context.expires_at,
            self.secret_key.as_bytes(),
        );
        let encoded_token = token.encode()?;

        Ok((encoded_token, context))
    }

    pub async fn get_operation<T: DeserializeOwned>(
        &self,
        continuation_token: &str,
    ) -> Result<(OperationContext, T), SessionError> {
        let token = ContinuationToken::decode(continuation_token, self.secret_key.as_bytes())?;
        let context = self.operation_store.get(&token.operation_id).await?;

        let typed_data: T = serde_json::from_value(context.data.clone())?;

        Ok((context, typed_data))
    }

    pub async fn complete_operation(&self, continuation_token: &str) -> Result<(), SessionError> {
        let token = ContinuationToken::decode(continuation_token, self.secret_key.as_bytes())?;
        self.operation_store.delete(&token.operation_id).await?;

        Ok(())
    }
}
