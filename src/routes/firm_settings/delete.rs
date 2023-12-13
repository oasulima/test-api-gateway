use crate::external_services_client::{ServicesClient, ServicesEnum};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathInfo {
    firm_name: String,
}

pub async fn delete_firm_setting(
    services_client: web::Data<ServicesClient>,
    query_info: web::Path<PathInfo>,
) -> impl Responder {
    let response = services_client
        .delete(
            ServicesEnum::Locator,
            &format!("/api/settings/firm/{}", query_info.firm_name),
        )
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
