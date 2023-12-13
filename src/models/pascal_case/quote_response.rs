use serde::Deserialize;
use time::OffsetDateTime;

use crate::models::QuoteResponseStatusEnum;

use super::QuoteSourceInfoKafka;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuoteResponse {
    pub id: String,
    pub firm_id: String,
    pub account_id: String,
    pub symbol: String,
    pub status: QuoteResponseStatusEnum,
    pub error_message: Option<String>,
    pub req_qty: i32,
    pub fill_qty: Option<i32>,
    pub price: Option<f64>,
    pub sources: Option<Vec<QuoteSourceInfoKafka>>,
    #[serde(with = "time::serde::iso8601")]
    pub time: OffsetDateTime,
    pub details_json: String,
}
