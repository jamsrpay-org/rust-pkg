use redis_rs::RedisClient;
use serde_json::json;
use token_orchestrator::TokenOrchestrator;

mod continuation_token;
mod error;
mod operation_context;
mod operation_store;
mod token_orchestrator;

const SECRET_KEY: &str = "secret_key";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operation_data = json!({
        "email": "princeraj9137@gmail.com",
        "password":"jamsrworld"
    });

    let redis = RedisClient::new("redis://localhost:6379").await?;
    let token_orchestrator = TokenOrchestrator::new(redis, SECRET_KEY.to_ascii_lowercase());

    let data = token_orchestrator
        .initiate_operation("auth.register".to_string(), operation_data)
        .await?;
    dbg!(&data);

    // let data =

    Ok(())
}
