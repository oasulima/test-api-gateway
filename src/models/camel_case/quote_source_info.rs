use serde::{Deserialize, Serialize};

use crate::models::QuoteSourceInfoKafka;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteSourceInfo {
    pub provider: String,
    pub source: String,
    pub price: f64,
    pub qty: i32,
    pub user_price: f64,
    pub discounted_price: f64,
}

impl From<QuoteSourceInfoKafka> for QuoteSourceInfo {
    fn from(value: QuoteSourceInfoKafka) -> Self {
        Self {
            provider: value.provider,
            source: value.source,
            price: value.price,
            qty: value.qty,
            user_price: value.user_price,
            discounted_price: value.discounted_price,
        }
    }
}
