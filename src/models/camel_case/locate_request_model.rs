use serde::Serialize;
use time::OffsetDateTime;

use super::QuoteSourceInfo;

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocateRequestModel {
    pub id: String,
    pub account_id: String,
    pub firm_id: String,
    #[serde(with = "time::serde::iso8601")]
    pub time: OffsetDateTime,
    pub symbol: String,
    pub qty_req: i32,
    pub qty_offer: i32,
    pub price: f64,
    pub tz_price: f64,
    pub source: String,
    pub source_details: Vec<QuoteSourceInfo>,
}
