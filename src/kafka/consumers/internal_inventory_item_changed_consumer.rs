use crate::configuration::KafkaSettings;
use crate::models::{InternalInventoryItem, InternalInventoryItemKafka};
use crate::services::NotificationsService;
use actix_web::web;
use tokio_util::sync::CancellationToken;

use super::consume_messages;

pub async fn run_internal_inventory_item_changed_consumer(
    kafka_options: KafkaSettings,
    notification_service: web::Data<NotificationsService>,
    shutdown_cts: CancellationToken,
) {
    if let Err(e) = consume_messages(
        kafka_options.bootstrap_servers,
        kafka_options.internal_inventory_item_reporting_topic,
        kafka_options.group_id,
        shutdown_cts,
        |message| async {
            process_message(message, notification_service.clone()).await;
        },
    )
    .await
    {
        tracing::error!("Failed consuming messages: {}", e);
    }
}

async fn process_message(
    message_value: Vec<u8>,
    notification_service: web::Data<NotificationsService>,
) {
    let deserializer = &mut serde_json::Deserializer::from_slice(&message_value);
    let result: Result<InternalInventoryItemKafka, _> =
        serde_path_to_error::deserialize(deserializer);

    match result {
        Ok(message) => {
            notification_service
                .send_internal_inventory_item(InternalInventoryItem::from(message))
                .await
        }
        Err(err) => {
            tracing::error!("internal_inventory_item_changed_consumer: Couldn't parse InternalInventoryItemKafka: {:?}", err);
        }
    }
}
