use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FirmProviderSettingRequest {
    provider_id: String,
    provider_priority: Option<i32>,
    enabled: Option<bool>,
}
