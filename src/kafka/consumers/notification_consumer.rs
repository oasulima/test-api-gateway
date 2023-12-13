use super::consume_messages;
use crate::configuration::KafkaSettings;
use crate::models::{GroupedNotification, GroupedNotificationKafka};
use crate::services::NotificationsService;
use actix_web::web;
use tokio_util::sync::CancellationToken;

pub async fn run_notifications_consumer(
    kafka_options: KafkaSettings,
    notifications_service: web::Data<NotificationsService>,
    shutdown_cts: CancellationToken,
) {
    if let Err(e) = consume_messages(
        kafka_options.bootstrap_servers,
        kafka_options.notification_topic,
        kafka_options.group_id,
        shutdown_cts,
        |message| async {
            process_message(message, notifications_service.clone()).await;
        },
    )
    .await
    {
        tracing::error!("Failed consuming messages: {}", e);
    }
}

async fn process_message(
    message_value: Vec<u8>,
    notifications_service: web::Data<NotificationsService>,
) {
    let deserializer = &mut serde_json::Deserializer::from_slice(&message_value);
    let result: Result<Vec<GroupedNotificationKafka>, _> =
        serde_path_to_error::deserialize(deserializer);

    match result {
        Ok(message) => {
            let data_to_send: Vec<GroupedNotification> = message
                .iter()
                .map(|x| GroupedNotification::from(x.clone()))
                .collect();

            notifications_service.send_notifications(data_to_send).await;
        }
        Err(err) => {
            tracing::error!(
                "notifications_consumer: Couldn't parse GroupedNotificationKafka: {:?}",
                err
            );
        }
    }
}
