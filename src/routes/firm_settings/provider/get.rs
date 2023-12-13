use crate::external_services_client::{ServicesClient, ServicesEnum};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathInfo {
    firm_name: String,
}

pub async fn get_firm_provider_settings(
    services_client: web::Data<ServicesClient>,
    query_info: web::Path<PathInfo>,
) -> impl Responder {
    let response = services_client
        .get(
            ServicesEnum::Locator,
            &format!("/api/settings/firm/{}/provider", query_info.firm_name),
            &vec![],
        )
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
