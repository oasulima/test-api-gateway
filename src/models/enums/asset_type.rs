use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetType {
    Locate,
    PreBorrow,
}
