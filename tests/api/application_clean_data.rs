use crate::helpers::spawn_app;
use wiremock::http::Method;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn clean_all_services_data_returns_200_and_calls_services() {
    // Arrange
    let app = spawn_app().await;

    let _internal_inventory_server_mock_guard = Mock::given(path("/api/application/cleanData"))
        .and(method(Method::Post))
        .respond_with(ResponseTemplate::new(200))
        .named("Internal Inventory Mock Server")
        .expect(1)
        .mount(&app.internal_inventory_server)
        .await;

    let _order_book_server_mock_guard = Mock::given(path("/api/application/cleanData"))
        .and(method(Method::Post))
        .respond_with(ResponseTemplate::new(200))
        .named("Orderbook Mock Server")
        .expect(1)
        .mount(&app.order_book_server)
        .await;

    let _locator_server_mock_guard = Mock::given(path("/api/application/cleanData"))
        .and(method(Method::Post))
        .respond_with(ResponseTemplate::new(200))
        .named("Locator Mock Server")
        .expect(1)
        .mount(&app.locator_server)
        .await;

    // Act
    let response = app
        .post_application_clean_data()
        .await
        .error_for_status()
        .unwrap();

    let status = response.status();
    let body = response.text().await.unwrap();

    //Assert
    assert!(status.is_success());
    assert_eq!(body, "true");
}
