use rdkafka::{
    ClientConfig,
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    error::KafkaError,
};

pub async fn create_admin() -> Result<AdminClient<DefaultClientContext>, KafkaError> {
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set(
            "bootstrap.servers",
            "localhost:9194,localhost:9195,localhost:9196",
        )
        .create()?;

    let topics = vec![
        NewTopic::new("auth.events.v1", 10, TopicReplication::Fixed(3)),
        NewTopic::new("user.events.v1", 10, TopicReplication::Fixed(3)),
        NewTopic::new("mail.commands.v1", 10, TopicReplication::Fixed(3)),
    ];

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
