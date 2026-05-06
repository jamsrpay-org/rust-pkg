use rdkafka::{
    ClientConfig,
    admin::{AdminClient, AdminOptions, NewTopic},
    client::DefaultClientContext,
    error::KafkaError,
};
use std::time::Duration;

pub async fn create_topics(
    brokers: &str,
    topics: Vec<NewTopic<'_>>,
) -> Result<AdminClient<DefaultClientContext>, KafkaError> {
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("socket.timeout.ms", "5000")
        .set("request.timeout.ms", "5000")
        .set("metadata.request.timeout.ms", "3000")
        .create()?;

    let results = admin_client
        .create_topics(
            &topics,
            &AdminOptions::new()
                .request_timeout(Some(Duration::from_secs(5)))
                .operation_timeout(Some(Duration::from_secs(5))),
        )
        .await?;

    for result in results {
        match result {
            Ok(topic) => println!("Created topic: {}", topic),
            Err((topic, err)) => println!("Failed to create: {}: {:?}", topic, err),
        }
    }

    Ok(admin_client)
}
