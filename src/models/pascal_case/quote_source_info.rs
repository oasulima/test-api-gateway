use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct QuoteSourceInfoKafka {
    pub provider: String,
    pub source: String,
    pub price: f64,
    pub qty: i32,
    pub user_price: f64,
    pub discounted_price: f64,
}
