use crate::helpers::spawn_app;

#[tokio::test]
async fn get_running_env_returns_local() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_running_env().await.error_for_status().unwrap();

    let body: String = response.text().await.unwrap();

    //Assert
    assert_eq!(body, "Local");
}
