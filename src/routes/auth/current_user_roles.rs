use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct UserRolesResponse {
    #[serde(rename = "isAdmin")]
    is_admin: bool,
    #[serde(rename = "isViewer")]
    is_viewer: bool,
    #[serde(rename = "isProvider")]
    is_provider: bool,
    #[serde(rename = "providerId")]
    provider_id: Option<String>,
}

pub async fn current_user_roles() -> impl Responder {
    let data = UserRolesResponse {
        is_admin: true,
        is_viewer: false,
        is_provider: false,
        provider_id: None,
    };
    web::Json(data)
}
