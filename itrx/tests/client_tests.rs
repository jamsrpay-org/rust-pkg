use std::collections::BTreeMap;

use itrx::{
    client::{ItrxClient, ItrxConfig, MAINNET_BASE_URL, TESTNET_BASE_URL},
    types::*,
};

// ─── Unit: Signature ────────────────────────────────────────────────

#[test]
fn test_sign_matches_docs_example() {
    // Values taken from https://develop.itrx.io/general/sign.html
    let config = ItrxConfig {
        base_url: MAINNET_BASE_URL.to_string(),
        api_key: "B433BFF1CDE7450AA38A56BEAC690DD4".to_string(),
        api_secret: "0285A2741D0E76E2E187260EB23E51851D48403A756333E7D0CF845406ABF3E8"
            .to_string(),
    };
    let client = ItrxClient::new(config);

    let timestamp = "1686796826";
    let json_body = r#"{"callback_url":"http://{mydomain}/callback","energy_amount":32000,"out_trade_no":"123456","period":"1D","receive_address":"TExWKszFWYTKZH8LYiovAPKzS3L9MLZ4kw"}"#;

    let sig = client.sign(timestamp, json_body).unwrap();
    assert_eq!(
        sig,
        "07c125cabfe006614a02131f927cca9e7a654f7644a817b155fad591e1c50f7b"
    );
}

#[test]
fn test_sign_deterministic() {
    let config = ItrxConfig::testnet("KEY", "SECRET");
    let client = ItrxClient::new(config);

    let sig1 = client.sign("12345", r#"{"a":1}"#).unwrap();
    let sig2 = client.sign("12345", r#"{"a":1}"#).unwrap();
    assert_eq!(sig1, sig2);
}

// ─── Unit: JSON body sorting ────────────────────────────────────────

#[test]
fn test_order_params_serialize_sorted() {
    let params = CreateOrderParams {
        energy_amount: 32000,
        period: "1D",
        receive_address: "TExWKszFWYTKZH8LYiovAPKzS3L9MLZ4kw",
        callback_url: Some("http://{mydomain}/callback"),
        out_trade_no: Some("123456"),
    };

    let value = serde_json::to_value(&params).unwrap();
    let sorted: BTreeMap<String, serde_json::Value> = serde_json::from_value(value).unwrap();
    let json = serde_json::to_string(&sorted).unwrap();

    // Keys must be alphabetically sorted
    let keys: Vec<&str> = sorted.keys().map(|k| k.as_str()).collect();
    assert_eq!(
        keys,
        vec![
            "callback_url",
            "energy_amount",
            "out_trade_no",
            "period",
            "receive_address",
        ]
    );

    // The JSON body should match the docs example
    assert_eq!(
        json,
        r#"{"callback_url":"http://{mydomain}/callback","energy_amount":32000,"out_trade_no":"123456","period":"1D","receive_address":"TExWKszFWYTKZH8LYiovAPKzS3L9MLZ4kw"}"#
    );
}

#[test]
fn test_order_params_skip_none_fields() {
    let params = CreateOrderParams {
        energy_amount: 32000,
        period: "1H",
        receive_address: "TAddr",
        callback_url: None,
        out_trade_no: None,
    };

    let json = serde_json::to_string(&params).unwrap();
    assert!(!json.contains("callback_url"));
    assert!(!json.contains("out_trade_no"));
}

// ─── Unit: Config constructors ──────────────────────────────────────

#[test]
fn test_mainnet_config() {
    let config = ItrxConfig::mainnet("key", "secret");
    assert_eq!(config.base_url, MAINNET_BASE_URL);
    assert_eq!(config.api_key, "key");
    assert_eq!(config.api_secret, "secret");
}

#[test]
fn test_testnet_config() {
    let config = ItrxConfig::testnet("key", "secret");
    assert_eq!(config.base_url, TESTNET_BASE_URL);
}

// ─── Unit: Deserialization ──────────────────────────────────────────

#[test]
fn test_deserialize_create_order_response() {
    let json = r#"{"errno":0,"serial":"7297a8a2a9e39b86fc5bad0d2e9edda2","amount":3120000,"balance":813900029257}"#;
    let envelope: ApiEnvelope<CreateOrderResponse> = serde_json::from_str(json).unwrap();

    assert_eq!(envelope.errno, 0);
    let data = envelope.data.unwrap();
    assert_eq!(data.serial, "7297a8a2a9e39b86fc5bad0d2e9edda2");
    assert_eq!(data.amount, 3120000);
    assert_eq!(data.balance, 813900029257);
}

#[test]
fn test_deserialize_estimate_response() {
    let json = r#"{"period":"3D","energy_amount":32000,"price":100,"total_price":10192000,"addition":600000}"#;
    let resp: EstimateEnergyResponse = serde_json::from_str(json).unwrap();

    assert_eq!(resp.energy_amount, 32000);
    assert_eq!(resp.period, "3D");
    assert_eq!(resp.price, 100);
    assert_eq!(resp.total_price, 10192000);
    assert_eq!(resp.addition, 600000);
}

#[test]
fn test_deserialize_estimate_response_no_addition() {
    let json = r#"{"period":"1H","energy_amount":65000,"price":80,"total_price":5200000}"#;
    let resp: EstimateEnergyResponse = serde_json::from_str(json).unwrap();

    assert_eq!(resp.addition, 0); // defaults to 0
}

#[test]
fn test_deserialize_balance_response() {
    let json = r#"{
        "platform_avail_energy": 603249,
        "platform_max_energy": 329009,
        "minimum_order_energy": 32000,
        "maximum_order_energy": 100000000,
        "small_amount": 50000,
        "small_addition": 0.6,
        "usdt_energy_need_old": 32000,
        "usdt_energy_need_new": 65000,
        "tiered_pricing": [
            {"period": 0, "price": 100},
            {"period": 1, "price": 200},
            {"period": 3, "price": 152},
            {"period": 30, "price": 124}
        ],
        "balance": 813892429257
    }"#;

    let resp: BalanceResponse = serde_json::from_str(json).unwrap();

    assert_eq!(resp.platform_avail_energy, 603249);
    assert_eq!(resp.balance, 813892429257);
    assert_eq!(resp.tiered_pricing.len(), 4);
    assert_eq!(resp.tiered_pricing[0].period, 0);
    assert_eq!(resp.tiered_pricing[0].price, 100);
    assert_eq!(resp.small_addition, 0.6);
}

#[test]
fn test_deserialize_api_error_envelope() {
    let json = r#"{"errno":1,"message":"Insufficient balance"}"#;
    let envelope: ApiEnvelope<CreateOrderResponse> = serde_json::from_str(json).unwrap();

    assert_eq!(envelope.errno, 1);
    assert_eq!(
        envelope.message.as_deref(),
        Some("Insufficient balance")
    );
}

// ─── Integration (requires live API credentials) ────────────────────
// Set ITRX_API_KEY and ITRX_API_SECRET env vars to run these.

#[tokio::test]
#[ignore = "requires ITRX API credentials"]
async fn integration_get_balance() {
    let config = ItrxConfig::testnet(
        std::env::var("ITRX_API_KEY").unwrap(),
        std::env::var("ITRX_API_SECRET").unwrap(),
    );
    let client = ItrxClient::new(config);

    let balance = client.get_balance().await.unwrap();
    println!("Balance: {} sun ({} TRX)", balance.balance, balance.balance as f64 / 1_000_000.0);
    assert!(balance.minimum_order_energy > 0);
}

#[tokio::test]
#[ignore = "requires ITRX API credentials"]
async fn integration_estimate_energy() {
    let config = ItrxConfig::testnet(
        std::env::var("ITRX_API_KEY").unwrap(),
        std::env::var("ITRX_API_SECRET").unwrap(),
    );
    let client = ItrxClient::new(config);

    let estimate = client
        .estimate_energy_amount(&EstimateEnergyParams {
            period: "1H",
            energy_amount: Some(32000),
            to_address: None,
        })
        .await
        .unwrap();

    println!("Estimate: {:?}", estimate);
    assert!(estimate.total_price > 0);
    assert_eq!(estimate.energy_amount, 32000);
}

#[tokio::test]
#[ignore = "requires ITRX API credentials and spends balance"]
async fn integration_transfer_energy() {
    let config = ItrxConfig::testnet(
        std::env::var("ITRX_API_KEY").unwrap(),
        std::env::var("ITRX_API_SECRET").unwrap(),
    );
    let client = ItrxClient::new(config);

    let address = "TBpVomvaZtTgBpSpE5jnqYGj5Js1Ywj7NP";
    let result = client.transfer_energy(address, address).await.unwrap();
    println!("Order: {:?}", result);
    assert!(!result.serial.is_empty());
}
