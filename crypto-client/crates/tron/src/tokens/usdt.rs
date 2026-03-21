use crate::{client::TronClient, tokens::trc20::Trc20Token};

pub fn usdt_trc20(client: TronClient) -> Trc20Token {
    Trc20Token {
        symbol: "USDT",
        contract_address: "TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj",
        decimals: 6,
        client,
    }
}
