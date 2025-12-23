use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationContext {
    pub operation_id: String,
    pub operation_type: String,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl OperationContext {
    pub fn new(operation_type: String, data: serde_json::Value) -> Self {
        let operation_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        Self {
            operation_id,
            operation_type,
            data,
            created_at: now,
            expires_at: now + Duration::minutes(30),
        }
    }
}
