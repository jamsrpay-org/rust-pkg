use crate::{client::TronClient, error::TronClientError};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccountResource {
    #[serde(default, rename = "freeNetUsed")]
    pub free_net_used: u128,
    #[serde(default, rename = "freeNetLimit")]
    pub free_net_limit: u128,
    #[serde(default, rename = "NetUsed")]
    pub net_used: u128,
    #[serde(default, rename = "NetLimit")]
    pub net_limit: u128,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    #[serde(default)]
    pub balance: u128,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub create_time: usize,
}

impl TronClient {
    pub async fn get_account(&self, address: &str) -> Result<Account, TronClientError> {
        let url = format!("{}/getaccount", self.http_base_url);

        let body = serde_json::json!({
            "address": address,
            "visible": true,
        });

        let resp = self.client().post(&url).json(&body).send().await?;
        let json: Account = resp.json().await?;

        Ok(json)
    }

    pub async fn get_balance(&self, address: &str) -> Result<u128, TronClientError> {
        let account = self.get_account(address).await?;
        Ok(account.balance)
    }

    pub async fn get_account_resource(
        &self,
        address: &str,
    ) -> Result<AccountResource, TronClientError> {
        let url = format!("{}/getaccountresource", self.http_base_url);

        let body = serde_json::json!({
            "address": address,
            "visible": true,
        });

        let resp = self.client.post(&url).json(&body).send().await?;
        let json: AccountResource = resp.json().await?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::TronClient;

    #[tokio::test]
    async fn get_account_resource() {
        let client = TronClient::new("https://nile.trongrid.io/wallet");
        let address = "TAbZyApgqgDfnBvQ7B1BTAZ7sb1qD7LEQt";
        let resource = client.get_account_resource(address).await.unwrap();
        println!("{:?}", resource);
    }
}
