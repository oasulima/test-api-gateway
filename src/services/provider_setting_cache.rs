use crate::{
    constants::providers_ids,
    external_services_client::{ServicesClient, ServicesEnum},
    models::{ProviderSetting, ProviderSettingExtended},
};
use actix_web::web;
use std::collections::HashMap;

type ProviderIdType = String;

pub struct ProviderSettingCache {
    provider_settings: HashMap<ProviderIdType, ProviderSetting>,
}

impl ProviderSettingCache {
    pub async fn new(services_client: web::Data<ServicesClient>) -> Self {
        Self {
            provider_settings: get_fresh_settings(services_client).await,
        }
    }

    pub fn get_all(&self) -> Vec<ProviderSetting> {
        self.provider_settings.values().cloned().collect()
    }

    pub fn get_active_external_query(&self) -> Vec<ProviderSetting> {
        self.provider_settings
            .values()
            .filter(|x| {
                x.active
                    && x.provider_id != providers_ids::INTERNAL_INVENTORY
                    && x.provider_id != providers_ids::ORDER_BOOK
            })
            .cloned()
            .collect()
    }

    pub fn get_provider_setting(&self, provider_id: String) -> Option<ProviderSetting> {
        let value = self.provider_settings.get(&provider_id);
        value.cloned()
    }
}

async fn get_fresh_settings(
    services_client: web::Data<ServicesClient>,
) -> HashMap<ProviderIdType, ProviderSetting> {
    let fresh_settings = get_settings_from_locator(services_client).await;
    let mut provider_settings: HashMap<ProviderIdType, ProviderSetting> = HashMap::new();

    for settings in fresh_settings {
        let provider_id = settings.provider_id.clone();
        provider_settings.insert(provider_id, map(settings));
    }

    provider_settings
}

fn map(extended: ProviderSettingExtended) -> ProviderSetting {
    ProviderSetting {
        provider_id: extended.provider_id,
        name: extended.name,
        multiplier: extended.multiplier,
        vig: extended.vig,
        discount: extended.discount,
        dynamic_price_multiplier: extended.dynamic_price_multiplier,
        active: extended.active,
        quote_request_topic: extended.quote_request_topic,
        quote_response_topic: extended.quote_response_topic,
        buy_request_topic: extended.buy_request_topic,
        buy_response_topic: extended.buy_response_topic,
    }
}

async fn get_settings_from_locator(
    services_client: web::Data<ServicesClient>,
) -> Vec<ProviderSettingExtended> {
    let response = services_client
        .get(ServicesEnum::Locator, "/api/settings/provider", &vec![])
        .await
        .unwrap();

    response
        .json::<Vec<ProviderSettingExtended>>()
        .await
        .unwrap()
}
