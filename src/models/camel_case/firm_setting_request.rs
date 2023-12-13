use serde::{Deserialize, Serialize};

use super::FirmProviderSettingRequest;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FirmSettingRequest {
    name: String, 
    is_external: Option<bool>,
    multiplier: Option<f64>,
    vig: Option<f64>, 
    sell_back_discount: Option<f64>,
    active: Option<bool>, 
    firm_provider_settings: Vec<FirmProviderSettingRequest>,
}
