use jamsrpay_kafka::create_topics;
use rdkafka::admin::{NewTopic, TopicReplication};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let brokers = "localhost:9094";

    let shared_topics: Vec<&str> = vec![
        // Users
        "user.events.v1",
        "identity.events.v1",
    ];
    let topics: Vec<&str> = vec![
        // Billing
        "billing.invoice.events.v1",
        "billing.payment_intent.events.v1",
        "billing.deposit_wallet.events.v1",
        // Payouts
        "payout.events.v1",
        "payout.payout_wallet.events.v1",
        "payout.gas_wallet.events.v1",
        "payout.funding_wallet.events.v1",
        // Stores
        "store.store.events.v1",
        "store.api_key.events.v1",
        "store.store_currency.events.v1",
    ];

    let chain_topics: Vec<&str> = vec![
        // Tron
        "tron.transfers.native.v1",
        "tron.transfers.token.v1",
        "tron.blocks.v1",
        // BSC
        "bsc.transfers.native.v1",
        "bsc.transfers.token.v1",
        "bsc.blocks.v1",
        // ETH
        "eth.transfers.native.v1",
        "eth.transfers.token.v1",
        "eth.blocks.v1",
        // Polygon
        "pol.transfers.native.v1",
        "pol.transfers.token.v1",
        "pol.blocks.v1",
        // BTC
        "btc.transfers.native.v1",
        "btc.transfers.token.v1",
        "btc.blocks.v1",
        // LTC
        "ltc.transfers.native.v1",
        "ltc.transfers.token.v1",
        "ltc.blocks.v1",
        // SOL
        "sol.transfers.native.v1",
        "sol.transfers.token.v1",
        "sol.blocks.v1",
        // Blockchain events
        "blockchain.invoice.payment_detected.v1",
        "blockchain.invoice.payment_confirmed.v1",
        "blockchain.gas_wallet.transaction_detected.v1",
        "blockchain.payout.transaction_detected.v1",
        "blockchain.payout.native_transfer_detected.v1",
        "blockchain.payout.transaction_status_changed.v1",
        "blockchain.payout_funding_wallet.transaction_detected.v1",
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
