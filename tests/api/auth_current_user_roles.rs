use crate::helpers::spawn_app;

#[tokio::test]
async fn current_user_is_admin() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .get_current_user_roles()
        .await
        .error_for_status()
        .unwrap();

    let body: serde_json::Value = response.json::<serde_json::Value>().await.unwrap();

    //Assert
    assert_eq!(body["isAdmin"], true, "isAdmin");
    assert_eq!(body["isViewer"], false, "isViewer");
    assert_eq!(body["isProvider"], false, "isProvider");
    assert!(body["providerId"].is_null(), "providerId");
}
