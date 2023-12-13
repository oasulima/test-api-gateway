use serde::Deserialize;
use time::OffsetDateTime;

use crate::models::AssetType;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProviderSymbolLocatesInfo {
    pub symbol: String,
    pub quantity: i32,
    pub price: f64,
    pub provider_id: String,
    pub provider_name: String,
    pub asset_type: AssetType,
    #[serde(with = "time::serde::iso8601")]
    pub date_time: OffsetDateTime,
}
