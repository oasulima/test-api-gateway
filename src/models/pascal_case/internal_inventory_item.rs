use serde::Deserialize;
use time::OffsetDateTime;

use crate::models::{AssetType, CreatingType, InternalInventoryItemState};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InternalInventoryItemKafka {
    pub id: String,
    pub version: i32,
    pub symbol: String,
    pub quantity: i32,
    pub sold_quantity: i32,
    pub price: f64,
    pub source: String,
    pub asset_type: AssetType,
    pub creating_type: CreatingType,
    pub tag: Option<String>,
    pub covered_inv_item_id: Option<String>,
    pub status: InternalInventoryItemState,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}
