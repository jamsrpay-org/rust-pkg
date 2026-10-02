use jamsrpay_infra::kafka_topics;
use jamsrpay_kafka::create_topics;
use rdkafka::admin::{NewTopic, TopicReplication};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let brokers = "localhost:9094";

    let shared_topics: Vec<&str> = vec![
        // Users
        kafka_topics::USER_EVENTS,
        kafka_topics::IDENTITY_EVENTS,
    ];
    let topics: Vec<&str> = vec![
        // Billing
        kafka_topics::INVOICE_EVENTS,
        kafka_topics::PAYMENT_INTENT_EVENTS,
        kafka_topics::DEPOSIT_WALLET_EVENTS,
        kafka_topics::PRICING_PLAN_PURCHASE_EVENTS,
        // Payouts
        kafka_topics::PAYOUT_EVENTS,
        kafka_topics::PAYOUT_WALLET_EVENTS,
        kafka_topics::GAS_WALLET_EVENTS,
        kafka_topics::PAYOUT_FUNDING_WALLET_EVENTS,
        // Stores
        kafka_topics::STORE_EVENTS,
        kafka_topics::API_KEY_EVENTS,
        kafka_topics::STORE_CURRENCY_EVENTS,
        // Wallet
        kafka_topics::WALLET_ACCOUNT_EVENTS,
        kafka_topics::BLOCKCHAIN_WALLET_EVENTS,
    ];

    let chain_topics: Vec<&str> = vec![
        // Tron
        kafka_topics::TRON_NATIVE_TRANSFERS,
        kafka_topics::TRON_TOKEN_TRANSFERS,
        kafka_topics::TRON_BLOCKS,
        // BSC
        kafka_topics::BSC_NATIVE_TRANSFERS,
        kafka_topics::BSC_TOKEN_TRANSFERS,
        kafka_topics::BSC_BLOCKS,
        // ETH
        kafka_topics::ETH_NATIVE_TRANSFERS,
        kafka_topics::ETH_TOKEN_TRANSFERS,
        kafka_topics::ETH_BLOCKS,
        // Polygon
        kafka_topics::POL_NATIVE_TRANSFERS,
        kafka_topics::POL_TOKEN_TRANSFERS,
        kafka_topics::POL_BLOCKS,
        // BTC
        kafka_topics::BTC_NATIVE_TRANSFERS,
        kafka_topics::BTC_TOKEN_TRANSFERS,
        kafka_topics::BTC_BLOCKS,
        // LTC
        kafka_topics::LTC_NATIVE_TRANSFERS,
        kafka_topics::LTC_TOKEN_TRANSFERS,
        kafka_topics::LTC_BLOCKS,
        // SOL
        kafka_topics::SOL_NATIVE_TRANSFERS,
        kafka_topics::SOL_TOKEN_TRANSFERS,
        kafka_topics::SOL_BLOCKS,
        // Blockchain events
        kafka_topics::BLOCKCHAIN_INVOICE_PAYMENT_DETECTED,
        kafka_topics::BLOCKCHAIN_INVOICE_PAYMENT_CONFIRMED,
        kafka_topics::BLOCKCHAIN_GAS_WALLET_TRANSACTION_DETECTED,
        kafka_topics::BLOCKCHAIN_PAYOUT_TRANSACTION_DETECTED,
        kafka_topics::BLOCKCHAIN_PAYOUT_NATIVE_TRANSFER_DETECTED,
        kafka_topics::BLOCKCHAIN_PAYOUT_TRANSACTION_STATUS_CHANGED,
        kafka_topics::BLOCKCHAIN_PAYOUT_FUNDING_WALLET_TRANSACTION_DETECTED,
    ];

    let topics = [shared_topics, topics, chain_topics].concat();
    let new_topics = topics
        .iter()
        .map(|topic| NewTopic::new(topic, 1, TopicReplication::Fixed(1)))
        .collect::<Vec<NewTopic>>();
    create_topics(brokers, new_topics).await?;

    let sandbox_topic_names: Vec<String> = topics
        .iter()
        .map(|topic| format!("sandbox.{topic}"))
        .collect();

    let sandbox_topics = sandbox_topic_names
        .iter()
        .map(|topic| NewTopic::new(topic, 1, TopicReplication::Fixed(1)))
        .collect::<Vec<NewTopic>>();
    create_topics(brokers, sandbox_topics).await?;
    Ok(())
}
