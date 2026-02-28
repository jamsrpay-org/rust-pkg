use jamsrpay_kafka::create_admin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_admin().await?;
    Ok(())
}
