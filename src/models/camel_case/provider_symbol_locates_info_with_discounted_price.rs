use crate::models::AssetType;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSymbolLocatesInfoWithDiscountedPrice {
    pub symbol: String,
    pub quantity: i32,
    pub price: f64,
    pub discounted_price: f64,
    pub provider_id: String,
    pub provider_name: String,
    pub asset_type: AssetType,
    #[serde(with = "time::serde::iso8601")]
    pub date_time: OffsetDateTime,
}
