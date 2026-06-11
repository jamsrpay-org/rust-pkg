use crate::money::Money;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayMoney {
    money: Money,
    currency: String,
}

impl Deref for DisplayMoney {
    type Target = Money;

    fn deref(&self) -> &Self::Target {
        &self.money
    }
}

impl DisplayMoney {
    pub fn new(money: Money, currency: String) -> Self {
        Self { money, currency }
    }

    pub fn money(&self) -> &Money {
        &self.money
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }
}
