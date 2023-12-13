use crate::configuration::KafkaSettings;
use crate::models::{ProviderSymbolLocatesInfo, ProviderSymbolLocatesInfoWithDiscountedPrice};
use crate::services::{NotificationsService, ProviderSettingCache};
use actix_web::web;
use tokio_util::sync::CancellationToken;

use super::consume_messages;

pub async fn run_provide_locate_symbol_info_consumer(
    kafka_options: KafkaSettings,
    provider_setting_cache: web::Data<ProviderSettingCache>,
    notifications_service: web::Data<NotificationsService>,
    shutdown_cts: CancellationToken,
) {
    if let Err(e) = consume_messages(
        kafka_options.bootstrap_servers,
        kafka_options.provider_symbol_indo_topic,
        kafka_options.group_id,
        shutdown_cts,
        |message| async {
            process_message(
                message,
                provider_setting_cache.clone(),
                notifications_service.clone(),
            )
            .await;
        },
    )
    .await
    {
        tracing::error!("Failed consuming messages: {}", e);
    }
}

async fn process_message(
    message_value: Vec<u8>,
    provider_setting_cache: web::Data<ProviderSettingCache>,
    notifications_service: web::Data<NotificationsService>,
) {
    let deserializer = &mut serde_json::Deserializer::from_slice(&message_value);
    let result: Result<ProviderSymbolLocatesInfo, _> =
        serde_path_to_error::deserialize(deserializer);

    match result {
        Ok(message) => {
            let mut info = map(message);
            let provider_settings =
                provider_setting_cache.get_provider_setting(info.provider_id.clone());
            if let Some(provider_settings) = provider_settings {
                info.discounted_price = info.price * (1.0 - provider_settings.discount);
            }
            notifications_service
                .send_external_provider_quote_response(info)
                .await;
        }
        Err(err) => {
            tracing::error!(
                "provide_locate_symbol_info_consumer: Couldn't parse ProviderSymbolLocatesInfo: {:?}",
                err
            );
        }
    }
}

fn map(message: ProviderSymbolLocatesInfo) -> ProviderSymbolLocatesInfoWithDiscountedPrice {
    ProviderSymbolLocatesInfoWithDiscountedPrice {
        provider_id: message.provider_id,
        price: message.price,
        quantity: message.quantity,
        symbol: message.symbol,
        date_time: message.date_time,
        provider_name: message.provider_name,
        asset_type: message.asset_type,
        discounted_price: -1.0,
    }
}
