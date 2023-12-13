use crate::services::{MessageHandler, NotificationsService};
use actix_web::{web, Responder};
use std::time::Duration;

pub async fn handle_new_sse_client(
    notification_service: web::Data<NotificationsService>,
    message_handler: web::Data<MessageHandler>,
) -> impl Responder {
    tracing::info!("handle_new_sse_client");
    let (sender_id, sender, sse) = notification_service.add_user_to_provider_group();

    actix_web::rt::spawn(async move {
        let locate_requests = message_handler.get_locate_requests_history();
        let locates = message_handler.get_locates_history();

        notification_service
            .send_locate_requests_history_to_client(sender_id, sender.clone(), &locate_requests)
            .await;
        notification_service
            .send_locates_history_to_client(sender_id, sender.clone(), &locates)
            .await;
    });

    sse.with_keep_alive(Duration::from_secs(3))
}
