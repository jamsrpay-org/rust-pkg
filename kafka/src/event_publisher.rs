use prost::{Message, Name};
use rdkafka::{
    message::{Header, OwnedHeaders},
    producer::{FutureProducer, FutureRecord},
};
use std::time::Duration;

// Error types
#[derive(Debug, thiserror::Error)]
pub enum EventPublishError {
    #[error("Failed to encode event: {0}")]
    EncodeError(#[from] prost::EncodeError),

    #[error("Failed to publish to Kafka: {0}")]
    KafkaError(String),

    #[error("Event validation failed: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, EventPublishError>;

// Typed event publisher
#[derive(Clone)]
pub struct EventPublisher {
    producer: FutureProducer,
    timeout: Duration,
    service_name: String,
    topic: String,
}

impl EventPublisher {
    pub fn new(producer: FutureProducer, service_name: String, topic: String) -> Self {
        let timeout = Duration::from_secs(10);
        EventPublisher {
            producer,
            timeout,
            service_name,
            topic,
        }
    }

    pub async fn publish_with_key<E: Message + Name>(&self, event: E, key: &str) -> Result<()> {
        let payload = event.encode_to_vec();
        let headers = OwnedHeaders::new()
            .insert(Header {
                key: "event_type",
                value: Some(&E::type_url()),
            })
            .insert(Header {
                key: "source",
                value: Some(&self.service_name),
            })
            .insert(Header {
                key: "content_type",
                value: Some("application/protobuf"),
            });

        let record = FutureRecord::to(&self.topic)
            .headers(headers)
            .payload(&payload)
            .key(key);

        match self.producer.send(record, self.timeout).await {
            Ok(delivery) => {
                tracing::info!(
                    topic = self.topic,
                    partition = delivery.partition,
                    offset = delivery.offset,
                    "Event published successfully"
                );
                Ok(())
            }
            Err((err, _)) => {
                tracing::error!(topic = self.topic, error = %err, "Failed to publish event");
                Err(EventPublishError::KafkaError(err.to_string()))
            }
        }
    }
}
