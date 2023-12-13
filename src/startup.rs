use crate::routes::{
    add_firm_setting, clean_all_services_data, current_user_roles, delete_firm_setting,
    get_default_symbol_setting, get_firm_provider_settings, get_firm_settings,
    get_internal_inventory, get_internal_inventory_history, get_provider_settings, get_running_env,
    get_symbol_availability_type, handle_new_sse_client, quote_external_providers,
    replace_firm_provider_settings, update_default_symbol_setting, update_firm_setting,
};
use crate::services::{MessageHandler, NotificationsService, TimeService};
use crate::{configuration::Settings, external_services_client::ServicesClient};
use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
use time::Duration;
use tracing_actix_web::TracingLogger;

pub struct Application {
    pub port: u16,
    pub server: Server,
    pub injections: Injections,
}

pub struct Injections {
    pub services_client: web::Data<ServicesClient>,
    pub time_service: web::Data<TimeService>,
    pub notifications_service: web::Data<NotificationsService>,
    pub message_handler: web::Data<MessageHandler>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let services_client = web::Data::new(ServicesClient::new(configuration.services));
        let time_service = web::Data::new(TimeService::new(Duration::hours(7)));

        let notifications_service = web::Data::new(NotificationsService::new());
        let message_handler = web::Data::new(MessageHandler::new(
            notifications_service.clone(),
            time_service.clone(),
            services_client.clone(),
        ));

        let listener = TcpListener::bind(address).expect(&format!(
            "Failed to bind {}:{}",
            configuration.application.host, configuration.application.port
        ));

        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            services_client.clone(),
            message_handler.clone(),
            notifications_service.clone(),
            time_service.clone(),
        )
        .await?;

        let injections = Injections {
            services_client,
            time_service,
            message_handler,
            notifications_service,
        };

        Ok(Self {
            port,
            server,
            injections,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

pub async fn run(
    listener: TcpListener,
    services_client: web::Data<ServicesClient>,
    message_handler: web::Data<MessageHandler>,
    notification_service: web::Data<NotificationsService>,
    time_service: web::Data<TimeService>,
) -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8001")
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .route("/sse", web::get().to(handle_new_sse_client))
            .service(routes())
            .app_data(services_client.clone())
            .app_data(message_handler.clone())
            .app_data(notification_service.clone())
            .app_data(time_service.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[rustfmt::skip]
fn routes() -> actix_web::Scope {
    web::scope("/api")
            .route("/application/cleandata/allServices", web::post().to(clean_all_services_data))
            .route("/application/running-env", web::get().to(get_running_env))
            .route("/auth/roles", web::get().to(current_user_roles))
            .route("/internal-inventory", web::get().to(get_internal_inventory))
            .route("/internal-inventory/items/history", web::get().to(get_internal_inventory_history))
            .route("/quote/external-providers", web::post().to(quote_external_providers))
            .route("/settings/default/symbol", web::get().to(get_default_symbol_setting))
            .route("/settings/default/symbol", web::put().to(update_default_symbol_setting))
            .route("/settings/firm", web::get().to(get_firm_settings))
            .route("/settings/firm", web::post().to(add_firm_setting))
            .route("/settings/firm", web::put().to(update_firm_setting))
            .route("/settings/firm/{firm_name}", web::delete().to(delete_firm_setting))
            .route("/settings/firm/{firm_name}/provider", web::get().to(get_firm_provider_settings))
            .route("/settings/firm/{firm_name}/provider", web::post().to(replace_firm_provider_settings))
            .route("/settings/provider", web::get().to(get_provider_settings))
            .route("/symbols/availability", web::get().to(get_symbol_availability_type))
}
