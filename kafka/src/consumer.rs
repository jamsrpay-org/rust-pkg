use rdkafka::{ClientConfig, consumer::StreamConsumer, error::KafkaError};

pub fn create_consumer(brokers: &str, group_id: &str) -> Result<StreamConsumer, KafkaError> {
    let consumer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            brokers,
        )
        .set("group.id", group_id)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .set("max.poll.interval.ms", "600000")
        .set("session.timeout.ms", "10000")
        .set("heartbeat.interval.ms", "3000")
        .set("fetch.min.bytes", "1")
        .set("fetch.wait.max.ms", "500")
        .create()?;

    Ok(consumer)
}
