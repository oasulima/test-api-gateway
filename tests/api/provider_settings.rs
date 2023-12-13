use crate::helpers::spawn_app;
use serde_json::json;
use wiremock::http::Method;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn get_provider_settings() {
    // Arrange
    let app = spawn_app().await;

    let expected_body = json!([{"autoDisabled":null,"providerId":"II","name":"InternalInventory","multiplier":1.00000,"vig":0.0,"discount":0.0,"dynamicPriceMultiplier":null,"active":true,"quoteRequestTopic":"local.internalinventory_quote_request","quoteResponseTopic":"local.internalinventory_quote_response","buyRequestTopic":"local.internalinventory_buy_request","buyResponseTopic":"local.internalinventory_buy_response"},{"autoDisabled":null,"providerId":"ORBK","name":"OrderBook","multiplier":1.00000,"vig":0.0,"discount":0.0,"dynamicPriceMultiplier":null,"active":true,"quoteRequestTopic":"local.orderbook_quote_request","quoteResponseTopic":"local.orderbook_quote_response","buyRequestTopic":"local.orderbook_buy_request","buyResponseTopic":"local.orderbook_buy_response"},{"autoDisabled":null,"providerId":"FPMOCK","name":"fake_provider","multiplier":1.00000,"vig":0.0,"discount":0.0,"dynamicPriceMultiplier":null,"active":true,"quoteRequestTopic":"local.fake_provider_mock_quote_request","quoteResponseTopic":"local.fake_provider_mock_quote_response","buyRequestTopic":"local.fake_provider_mock_buy_request","buyResponseTopic":"local.fake_provider_mock_buy_response"}]);

    let _locator_server_mock_guard = Mock::given(path("/api/settings/provider"))
        .and(method(Method::Get))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_body))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app.get_provider_settings().await.error_for_status().unwrap();

    //Assert
    let response_body = response.json::<serde_json::Value>().await;

    assert!(response_body.is_ok());

    let response_body = response_body.unwrap();
    assert_eq!(response_body, expected_body)
}
