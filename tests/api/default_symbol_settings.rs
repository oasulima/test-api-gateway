//{"minUserPrice":0.00010,"vig":0.0,"multiplier":1.00000,"oneTimePreBorrowDiscount":0.0,"oneTimeLocateDiscount":0.0}

use crate::helpers::spawn_app;
use serde_json::json;
use wiremock::http::Method;
use wiremock::matchers::body_json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn get_default_symbol_settings() {
    // Arrange
    let app = spawn_app().await;

    let expected_body = json!({"minUserPrice":0.00010,"vig":0.0,"multiplier":1.00000,"oneTimePreBorrowDiscount":0.0,"oneTimeLocateDiscount":0.0});

    let _locator_server_mock_guard = Mock::given(path("/api/settings/default/symbol"))
        .and(method(Method::Get))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_body))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app
        .get_default_symbol_setting()
        .await
        .error_for_status()
        .unwrap();

    //Assert

    let response_body = response.json::<serde_json::Value>().await;

    assert!(response_body.is_ok());

    let response_body = response_body.unwrap();
    assert_eq!(response_body, expected_body)
}

#[tokio::test]
async fn update_default_symbol_settings() {
    // Arrange
    let app = spawn_app().await;

    let expected_body = json!({"minUserPrice":0.00010,"vig":0.0,"multiplier":1.00000,"oneTimePreBorrowDiscount":0.0,"oneTimeLocateDiscount":0.0});

    let _locator_server_mock_guard = Mock::given(path("/api/settings/default/symbol"))
        .and(method(Method::Put))
        .and(body_json(&expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_body))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app
        .update_default_symbol_setting(&expected_body)
        .await
        .error_for_status()
        .unwrap();

    //Assert
    let response_body = response.json::<serde_json::Value>().await;

    assert!(response_body.is_ok());

    let response_body = response_body.unwrap();
    assert_eq!(response_body, expected_body)
}
