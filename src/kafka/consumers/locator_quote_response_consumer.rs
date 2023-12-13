use crate::configuration::KafkaSettings;
use crate::models::{LocatorQuoteResponse, QuoteResponse};
use crate::services::{MessageHandler, NotificationsService};
use actix_web::web;
use tokio_util::sync::CancellationToken;

use super::consume_messages;
pub async fn run_locator_quote_response_consumer(
    kafka_options: KafkaSettings,
    message_handler: web::Data<MessageHandler>,
    notification_service: web::Data<NotificationsService>,
    shutdown_cts: CancellationToken,
) {
    if let Err(e) = consume_messages(
        kafka_options.bootstrap_servers,
        kafka_options.locator_quote_response_topic,
        kafka_options.group_id,
        shutdown_cts,
        |message| async {
            process_message(
                message,
                message_handler.clone(),
                notification_service.clone(),
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
    message_handler: web::Data<MessageHandler>,
    notification_service: web::Data<NotificationsService>,
) {
    let deserializer = &mut serde_json::Deserializer::from_slice(&message_value);
    let result: Result<QuoteResponse, _> = serde_path_to_error::deserialize(deserializer);

    match result {
        Ok(message) => {
            message_handler
                .handler_quote_response(LocatorQuoteResponse::from(message), notification_service)
                .await
        }
        Err(err) => {
            tracing::error!(
                "locator_quote_response_consumer: Couldn't parse QuoteResponse: {:?}",
                err
            );
        }
    }
}
