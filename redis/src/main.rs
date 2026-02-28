use client::RedisClient;
use error::RedisClientError;

mod client;
mod error;

// Usage example
#[tokio::main]
async fn main() -> Result<(), RedisClientError> {
    let client = RedisClient::new("redis://127.0.0.1:6379/").await?;

    // Set with expiry
    client.set("session:user123", "token_abc").await?;

    // Get value
    if let Some(token) = client.get::<_, String>("session:user123").await? {
        println!("Token: {}", token);
    }

    // Increment counter
    let visits = client.increment("page_views", 1).await?;
    println!("Page views: {}", visits);

    let visits2 = client.get::<_, u64>("page_views").await?;
    println!("Page view:{:?}", visits2);

    // Pipeline multiple sets
    client
        .pipeline_set_multiple(&[("key1", "value1"), ("key2", "value2"), ("key3", "value3")])
        .await?;

    println!(
        "Value of key1: {:#?}",
        client.get::<_, String>("key1").await?
    );

    // Health check
    client.ping().await?;

    // otp types types
    client.set_ex("1006090:otp", 123456, 300).await?;

    let otp = client.get::<_, u32>("1006090:otp").await?;
    println!("OTP: {:#?}", otp);

    Ok(())
}
