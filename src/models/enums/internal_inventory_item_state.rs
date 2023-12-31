use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum InternalInventoryItemState {
    Active,
    Inactive,
    Deleted,
}
