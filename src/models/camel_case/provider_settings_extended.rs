use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSetting {
    pub provider_id: String,
    pub name: String,
    pub multiplier: f64,
    pub vig: f64,
    pub discount: f64,
    pub dynamic_price_multiplier: Option<f64>,
    pub active: bool,
    pub quote_request_topic: Option<String>,
    pub quote_response_topic: Option<String>,
    pub buy_request_topic: Option<String>,
    pub buy_response_topic: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettingExtended {
    pub provider_id: String,
    pub name: String,
    pub multiplier: f64,
    pub vig: f64,
    pub discount: f64,
    pub dynamic_price_multiplier: Option<f64>,
    pub active: bool,
    pub quote_request_topic: Option<String>,
    pub quote_response_topic: Option<String>,
    pub buy_request_topic: Option<String>,
    pub buy_response_topic: Option<String>,
    pub auto_disabled: Option<AutoDisabledInfo>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoDisabledInfo {
    pub symbols: Option<Vec<String>>,
}
