use serde::Serialize;
use time::OffsetDateTime;

use crate::models::{
    AssetType, CreatingType, InternalInventoryItemKafka, InternalInventoryItemState,
};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InternalInventoryItem {
    pub id: String,
    pub version: i32,
    pub symbol: String,
    pub quantity: i32,
    pub sold_quantity: i32,
    pub price: f64,
    pub source: String,
    pub asset_type: AssetType,
    pub creating_type: CreatingType,
    pub tag: String,
    pub covered_inv_item_id: String,
    pub status: InternalInventoryItemState,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

impl From<InternalInventoryItemKafka> for InternalInventoryItem {
    fn from(value: InternalInventoryItemKafka) -> Self {
        Self {
            id: value.id,
            version: value.version,
            symbol: value.symbol,
            quantity: value.quantity,
            sold_quantity: value.sold_quantity,
            price: value.price,
            source: value.source,
            asset_type: value.asset_type,
            creating_type: value.creating_type,
            tag: value.tag.unwrap_or_default(),
            covered_inv_item_id: value.covered_inv_item_id.unwrap_or_default(),
            status: value.status,
            created_at: value.created_at,
        }
    }
}
