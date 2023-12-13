use locator_admin_rust::startup::Application;
use locator_admin_rust::{
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use wiremock::MockServer;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    // if std::env::var("TEST_LOG").is_ok() {
    let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
    init_subscriber(subscriber);
    // } else {
    //     let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
    //     init_subscriber(subscriber);
    // };
});

pub struct TestApp {
    pub port: u16,
    pub address: String,
    pub api_client: reqwest::Client,
    pub locator_server: MockServer,
    pub fake_provider_server: MockServer,
    pub internal_inventory_server: MockServer,
    pub reporting_server: MockServer,
    pub order_book_server: MockServer,
}

impl TestApp {
    pub async fn post_application_clean_data(&self) -> reqwest::Response {
        self.api_client
            .post(&format!(
                "{}/api/application/cleandata/allServices",
                &self.address
            ))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_running_env(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/api/application/running-env", &self.address))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_current_user_roles(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/api/auth/roles", &self.address))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_default_symbol_setting(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/api/settings/default/symbol", &self.address))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn update_default_symbol_setting<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .put(&format!("{}/api/settings/default/symbol", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_firm_settings(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/api/settings/firm", &self.address))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn add_firm_setting<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/api/settings/firm", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn update_firm_setting<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .put(&format!("{}/api/settings/firm", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_firm_setting(&self, firm_name: String) -> reqwest::Response {
        self.api_client
            .delete(&format!(
                "{}/api/settings/firm/{}",
                &self.address, firm_name
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_firm_provider_settings(&self, firm_name: String) -> reqwest::Response {
        self.api_client
            .get(&format!(
                "{}/api/settings/firm/{}/provider",
                &self.address, firm_name
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn replace_firm_provider_settings<Body>(
        &self,
        firm_name: String,
        body: &Body,
    ) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!(
                "{}/api/settings/firm/{}/provider",
                &self.address, firm_name
            ))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_provider_settings(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/api/settings/provider", &self.address))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&&TRACING);

    let locator_server = MockServer::start().await;
    let fake_provider_server = MockServer::start().await;
    let internal_inventory_server = MockServer::start().await;
    let reporting_server = MockServer::start().await;
    let order_book_server = MockServer::start().await;

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a random OS port
        c.application.port = 0;
        c.services.locator = locator_server.uri();
        c.services.fake_provider = fake_provider_server.uri();
        c.services.internal_inventory = internal_inventory_server.uri();
        c.services.reporting = reporting_server.uri();
        c.services.order_book = order_book_server.uri();

        c
    };

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    // Get the port before spawning the application
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        api_client: client,
        locator_server,
        fake_provider_server,
        internal_inventory_server,
        reporting_server,
        order_book_server,
    };

    test_app
}
