use jamsrpay_kafka::create_topics;
use rdkafka::admin::{NewTopic, TopicReplication};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let brokers = "localhost:9194,localhost:9195,localhost:9196";
    let topics = vec![
        NewTopic::new("auth.events.v1", 10, TopicReplication::Fixed(3)),
        NewTopic::new("user.events.v1", 10, TopicReplication::Fixed(3)),
        NewTopic::new("mail.commands.v1", 10, TopicReplication::Fixed(3)),
    ];
    create_topics(brokers, topics).await?;
    Ok(())
}
