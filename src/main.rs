use actix_web::web;
use locator_admin_rust::configuration::get_configuration;
use locator_admin_rust::kafka::{
    run_internal_inventory_item_changed_consumer, run_locator_quote_response_consumer,
    run_notifications_consumer, run_provide_locate_symbol_info_consumer,
};
use locator_admin_rust::services::ProviderSettingCache;
use locator_admin_rust::startup::Application;
use locator_admin_rust::telemetry::{get_subscriber, init_subscriber};
use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("locator_admin_rust".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;

    let services_client = application.injections.services_client.clone();
    //let time_service = application.injections.time_service.clone();
    let notifications_service = application.injections.notifications_service.clone();
    let message_handler = application.injections.message_handler.clone();

    let application_task = tokio::spawn(application.run_until_stopped());

    let kafka_options = configuration.kafka;

    let shutdown_cts = CancellationToken::new();

    let provider_setting_cache =
        web::Data::new(ProviderSettingCache::new(services_client.clone()).await);

    let provide_locate_symbol_info_consumer_task =
        tokio::spawn(run_provide_locate_symbol_info_consumer(
            kafka_options.clone(),
            provider_setting_cache.clone(),
            notifications_service.clone(),
            shutdown_cts.clone(),
        ));

    let notifications_consumer_task = tokio::spawn(run_notifications_consumer(
        kafka_options.clone(),
        notifications_service.clone(),
        shutdown_cts.clone(),
    ));

    let locator_quote_response_consumer_task = tokio::spawn(run_locator_quote_response_consumer(
        kafka_options.clone(),
        message_handler.clone(),
        notifications_service.clone(),
        shutdown_cts.clone(),
    ));

    let internal_inventory_item_changed_consumer_task =
        tokio::spawn(run_internal_inventory_item_changed_consumer(
            kafka_options.clone(),
            notifications_service.clone(),
            shutdown_cts.clone(),
        ));

    tokio::select! {
        o = application_task => {
            report_exit("API", o);
            shutdown_cts.cancel();
        },
        _ = provide_locate_symbol_info_consumer_task  => { tracing::info!("provide_locate_symbol_info_consumer has exited");shutdown_cts.cancel();}
        _ = notifications_consumer_task => { tracing::info!("notifications_consumer has exited");shutdown_cts.cancel();}
        _ = locator_quote_response_consumer_task => { tracing::info!("locator_quote_response_consumer has exited");shutdown_cts.cancel();}
        _ = internal_inventory_item_changed_consumer_task  => { tracing::info!("internal_inventory_item_changed_consumer has exited");shutdown_cts.cancel();}
    };

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
