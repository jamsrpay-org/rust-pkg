use rdkafka::{ClientConfig, error::KafkaError, producer::FutureProducer};

pub fn create_producer(brokers: &str) -> Result<FutureProducer, KafkaError> {
    let producer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            brokers,
        )
        .set("acks", "all")
        .set("message.timeout.ms", "5000")
        .set("batch.num.messages", "1000")
        .set("linger.ms", "10")
        .set("compression.codec", "gzip")
        .set("retries", "5")
        .create()?;

    tracing::info!("Kafka producer created");

    Ok(producer)
}
