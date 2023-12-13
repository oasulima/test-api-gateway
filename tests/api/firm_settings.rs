use crate::helpers::spawn_app;
use serde_json::json;
use wiremock::http::Method;
use wiremock::matchers::body_json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn get_firm_settings() {
    // Arrange
    let app = spawn_app().await;

    let expected_body = json!([{"name":"admin","isExternal":false,"multiplier":1.00000,"vig":0.0,"sellBackDiscount":0.0,"active":true,"firmProviderSettings":[{"firmName":"admin","providerId":"II","providerPriority":null,"enabled":false},{"firmName":"admin","providerId":"ORBK","providerPriority":null,"enabled":false}]},{"name":"internal","isExternal":false,"multiplier":1.00000,"vig":0.0,"sellBackDiscount":0.0,"active":true,"firmProviderSettings":[]}]);

    let _locator_server_mock_guard = Mock::given(path("/api/settings/firm"))
        .and(method(Method::Get))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_body))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app.get_firm_settings().await.error_for_status().unwrap();

    //Assert
    let response_body = response.json::<serde_json::Value>().await;

    assert!(response_body.is_ok());

    let response_body = response_body.unwrap();
    assert_eq!(response_body, expected_body)
}

#[tokio::test]
async fn add_firm_settings() {
    // Arrange
    let app = spawn_app().await;

    let expected_body = json!({"name":"admin","isExternal":false,"multiplier":1.0,"vig":2.0,"sellBackDiscount":0.0,"active":true,"firmProviderSettings":[]});

    let _locator_server_mock_guard = Mock::given(path("/api/settings/firm"))
        .and(method(Method::Post))
        .and(body_json(&expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_body))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app
        .add_firm_setting(&expected_body)
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
async fn update_firm_settings() {
    // Arrange
    let app = spawn_app().await;

    let expected_body = json!({"name":"admin","isExternal":false,"multiplier":1.0,"vig":2.0,"sellBackDiscount":0.0,"active":true,"firmProviderSettings":[{"providerId":"II","providerPriority":null,"enabled":false},{"providerId":"ORBK","providerPriority":null,"enabled":false}]});

    let _locator_server_mock_guard = Mock::given(path("/api/settings/firm"))
        .and(method(Method::Put))
        .and(body_json(&expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_body))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app
        .update_firm_setting(&expected_body)
        .await
        .error_for_status()
        .unwrap();

    //Assert
    let response_body = response.json::<serde_json::Value>().await;

    assert!(response_body.is_ok());

    let response_body = response_body.unwrap();
    assert_eq!(response_body, expected_body);
}

#[tokio::test]
async fn delete_firm_settings() {
    // Arrange
    let app = spawn_app().await;

    let expected_firm = "test_firm";

    let _locator_server_mock_guard = Mock::given(method(Method::Delete))
        .respond_with(ResponseTemplate::new(200))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app
        .delete_firm_setting(expected_firm.to_string())
        .await
        .error_for_status()
        .unwrap();

    //Assert
    let status = response.status();
    assert!(status.is_success());

    let received_requests = app.locator_server.received_requests().await.unwrap();

    let received_request = &received_requests[0];
    assert_eq!(
        received_request.url.path(),
        format!("/api/settings/firm/{}", expected_firm)
    );
}

#[tokio::test]
async fn get_firm_provider_settings() {
    // Arrange
    let app = spawn_app().await;

    let expected_firm = "test_firm";
    let expected_body = json!([{"firmName":"admin","providerId":"II","providerPriority":null,"enabled":false},{"firmName":"admin","providerId":"ORBK","providerPriority":null,"enabled":false}]);

    let _locator_server_mock_guard = Mock::given(method(Method::Get))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_body))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app
        .get_firm_provider_settings(expected_firm.to_string())
        .await
        .error_for_status()
        .unwrap();

    //Assert
    let status = response.status();
    assert!(status.is_success());

    let response_body = response.json::<serde_json::Value>().await;

    assert!(response_body.is_ok());

    let response_body = response_body.unwrap();

    assert_eq!(response_body, expected_body);

    let received_requests = app.locator_server.received_requests().await.unwrap();

    let received_request = &received_requests[0];
    assert_eq!(
        received_request.url.path(),
        format!("/api/settings/firm/{}/provider", expected_firm)
    );
}

#[tokio::test]
async fn replace_firm_provider_settings() {
    // Arrange
    let app = spawn_app().await;

    let expected_firm = "test_firm";
    let expected_body = json!([{"providerId":"II","providerPriority":null,"enabled":true},{"providerId":"ORBK","providerPriority":null,"enabled":false}]);

    let _locator_server_mock_guard = Mock::given(method(Method::Post))
        .and(body_json(&expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_body))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app
        .replace_firm_provider_settings(expected_firm.to_string(), &expected_body)
        .await
        .error_for_status()
        .unwrap();

    //Assert
    let status = response.status();
    assert!(status.is_success());

    // assert_gt!(response.content_length(), Some(0));

    let response_body = response.json::<serde_json::Value>().await;

    assert!(response_body.is_ok());

    let response_body = response_body.unwrap();

    assert_eq!(response_body, expected_body);

    let received_requests = app.locator_server.received_requests().await.unwrap();

    let received_request = &received_requests[0];
    assert_eq!(
        received_request.url.path(),
        format!("/api/settings/firm/{}/provider", expected_firm)
    );
}
