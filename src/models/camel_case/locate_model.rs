use serde::Serialize;
use time::OffsetDateTime;

use crate::models::{QuoteResponseStatusEnum, QuoteSourceInfo};

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocateModel {
    pub quote_id: String,
    pub account_id: String,
    pub firm_id: String,
    #[serde(with = "time::serde::iso8601")]
    pub time: OffsetDateTime,
    pub symbol: String,
    pub req_qty: i32,
    pub qty_fill: i32,
    pub tz_price: f64,
    pub user_price: f64,
    pub status: QuoteResponseStatusEnum,
    pub error_message: Option<String>,
    pub source: String,
    pub source_details: Vec<QuoteSourceInfo>,
}
