use rdkafka::{
    ClientConfig,
    admin::{AdminClient, AdminOptions, NewTopic},
    client::DefaultClientContext,
    error::KafkaError,
};

pub async fn create_topics(
    brokers: &str,
    topics: Vec<NewTopic<'_>>,
) -> Result<AdminClient<DefaultClientContext>, KafkaError> {
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()?;

    let results = admin_client
        .create_topics(&topics, &AdminOptions::new())
        .await?;

    for result in results {
        match result {
            Ok(topic) => println!("Created topic: {}", topic),
            Err((topic, err)) => println!("Failed to create: {}: {:?}", topic, err),
        }
    }

    Ok(admin_client)
}
