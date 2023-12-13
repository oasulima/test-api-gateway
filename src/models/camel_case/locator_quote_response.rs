use std::collections::HashSet;

use serde::Deserialize;
use time::OffsetDateTime;

use crate::models::{QuoteResponse, QuoteResponseStatusEnum, QuoteSourceInfo};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocatorQuoteResponse {
    pub id: String,
    pub firm_id: String,
    pub account_id: String,
    pub symbol: String,

    pub status: QuoteResponseStatusEnum,
    #[serde(with = "time::serde::iso8601")]
    pub time: OffsetDateTime,
    pub error_message: Option<String>,
    pub req_qty: i32,
    pub fill_qty: Option<i32>,
    pub price: Option<f64>,
    pub tz_price: f64,
    pub source: String,
    pub sources: Vec<QuoteSourceInfo>,
}

impl From<QuoteResponse> for LocatorQuoteResponse {
    fn from(value: QuoteResponse) -> Self {
        let sources = match value.sources {
            Some(sources) => sources,
            None => vec![],
        };

        let source = sources
            .iter()
            .map(|x| x.source.clone())
            .collect::<HashSet<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let mut tz_price = 0.0;

        if let Some(fill_qty) = value.fill_qty {
            if fill_qty > 0 {
                tz_price = sources
                    .iter()
                    .map(|x| x.discounted_price * x.qty as f64)
                    .sum::<f64>()
                    / fill_qty as f64;
            }
        }

        Self {
            id: value.id,
            firm_id: value.firm_id,
            account_id: value.account_id,
            symbol: value.symbol,
            status: value.status,
            time: value.time,
            error_message: value.error_message,
            req_qty: value.req_qty,
            fill_qty: value.fill_qty,
            price: value.price,
            tz_price,
            source,
            sources: sources
                .iter()
                .map(|x| QuoteSourceInfo::from(x.clone()))
                .collect(),
        }
    }
}
