use reqwest::Client;

use crate::configuration::ServicesSettings;

#[derive(Clone)]
pub struct ServicesClient {
    http_client: Client,
    services: ServicesSettings,
}

pub enum ServicesEnum {
    Locator,
    FakeProvider,
    InternalInventory,
    Reporting,
    OrderBook,
}

impl ServicesClient {
    pub fn new(services: ServicesSettings) -> Self {
        let http_client = Client::builder().build().unwrap();
        Self {
            http_client,
            services,
        }
    }

    pub async fn get(
        &self,
        service: ServicesEnum,
        path: &str,
        query_params: &Vec<(&str, String)>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", self.get_service_base_url(service), path);

        tracing::info!("url is {}", url);

        self.http_client
            .get(&url)
            .query(query_params)
            //.json(&request_body)
            .send()
            .await?
            .error_for_status()
    }

    pub async fn post<Body>(
        &self,
        service: ServicesEnum,
        path: &str,
        query_params: &Vec<(&str, String)>,
        request_body: &Body,
    ) -> Result<reqwest::Response, reqwest::Error>
    where
        Body: serde::Serialize,
    {
        let url = format!("{}{}", self.get_service_base_url(service), path);

        self.http_client
            .post(&url)
            .query(query_params)
            .json(&request_body)
            .send()
            .await?
            .error_for_status()
    }

    pub async fn put<Body>(
        &self,
        service: ServicesEnum,
        path: &str,
        request_body: &Body,
    ) -> Result<reqwest::Response, reqwest::Error>
    where
        Body: serde::Serialize,
    {
        let url = format!("{}{}", self.get_service_base_url(service), path);

        self.http_client
            .put(&url)
            .json(&request_body)
            .send()
            .await?
            .error_for_status()
    }

    pub async fn delete(
        &self,
        service: ServicesEnum,
        path: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", self.get_service_base_url(service), path);

        self.http_client
            .delete(&url)
            .send()
            .await?
            .error_for_status()
    }

    fn get_service_base_url(&self, service: ServicesEnum) -> String {
        match service {
            ServicesEnum::Locator => self.services.locator.to_string(),
            ServicesEnum::FakeProvider => self.services.fake_provider.to_string(),
            ServicesEnum::InternalInventory => self.services.internal_inventory.to_string(),
            ServicesEnum::Reporting => self.services.reporting.to_string(),
            ServicesEnum::OrderBook => self.services.order_book.to_string(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use claim::assert_err;
//     use claim::assert_ok;
//     use fake::faker::internet::en::SafeEmail;
//     use fake::faker::lorem::en::{Paragraph, Sentence};
//     use fake::{Fake, Faker};
//     use secrecy::Secret;
//     use wiremock::matchers::{any, header, header_exists, method, path};
//     use wiremock::Request;
//     use wiremock::{Mock, MockServer, ResponseTemplate};

//     struct SendEmailBodyMatcher;

//     impl wiremock::Match for SendEmailBodyMatcher {
//         fn matches(&self, request: &Request) -> bool {
//             // Try to parse the body as a JSON value
//             let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
//             if let Ok(body) = result {
//                 dbg!(&body);
//                 // Check that all the mandatory fields are populated
//                 // without inspecting the field values
//                 body.get("From").is_some()
//                     && body.get("To").is_some()
//                     && body.get("Subject").is_some()
//                     && body.get("HtmlBody").is_some()
//                     && body.get("TextBody").is_some()
//             } else {
//                 // If parsing failed, do not match the request
//                 false
//             }
//         }
//     }

//     /// Generate a random email subject
//     fn subject() -> String {
//         Sentence(1..2).fake()
//     }
//     /// Generate a random email content
//     fn content() -> String {
//         Paragraph(1..10).fake()
//     }
//     /// Generate a random subscriber email
//     fn email() -> SubscriberEmail {
//         SubscriberEmail::parse(SafeEmail().fake()).unwrap()
//     }
//     /// Get a test instance of `EmailClient`.
//     fn email_client(base_url: String) -> EmailClient {
//         EmailClient::new(
//             base_url,
//             email(),
//             Secret::new(Faker.fake()),
//             std::time::Duration::from_millis(200),
//         )
//     }

//     #[tokio::test]
//     async fn send_email_sends_the_expected_request() {
//         // Arrange
//         let mock_server = MockServer::start().await;
//         let email_client = email_client(mock_server.uri());

//         Mock::given(header_exists("X-Postmark-Server-Token"))
//             .and(header("Content-Type", "application/json"))
//             .and(path("/email"))
//             .and(method("POST"))
//             .and(SendEmailBodyMatcher)
//             .respond_with(ResponseTemplate::new(200))
//             .expect(1)
//             .mount(&mock_server)
//             .await;
//         // Act
//         let _ = email_client
//             .send_email(&email(), &subject(), &content(), &content())
//             .await;
//         // Assert
//         // Mock expectations are checked on drop
//     }

//     #[tokio::test]
//     async fn send_email_succeeds_if_the_server_returns_200() {
//         // Arrange
//         let mock_server = MockServer::start().await;
//         let email_client = email_client(mock_server.uri());

//         Mock::given(any())
//             .respond_with(ResponseTemplate::new(200))
//             .expect(1)
//             .mount(&mock_server)
//             .await;
//         // Act
//         let outcome = email_client
//             .send_email(&email(), &subject(), &content(), &content())
//             .await;
//         // Assert
//         assert_ok!(outcome);
//     }

//     #[tokio::test]
//     async fn send_email_fails_if_the_server_returns_500() {
//         // Arrange
//         let mock_server = MockServer::start().await;
//         let email_client = email_client(mock_server.uri());

//         Mock::given(any())
//             // Not a 200 anymore!
//             .respond_with(ResponseTemplate::new(500))
//             .expect(1)
//             .mount(&mock_server)
//             .await;
//         // Act
//         let outcome = email_client
//             .send_email(&email(), &subject(), &content(), &content())
//             .await;
//         // Assert
//         assert_err!(outcome);
//     }

//     #[tokio::test]
//     async fn send_email_times_out_if_the_server_takes_too_long() {
//         // Arrange
//         let mock_server = MockServer::start().await;
//         let email_client = email_client(mock_server.uri());

//         let response = ResponseTemplate::new(200)
//             // 3 minutes!
//             .set_delay(std::time::Duration::from_secs(180));

//         Mock::given(any())
//             .respond_with(response)
//             .expect(1)
//             .mount(&mock_server)
//             .await;
//         // Act
//         let outcome = email_client
//             .send_email(&email(), &subject(), &content(), &content())
//             .await;
//         // Assert
//         assert_err!(outcome);
//     }
// }
