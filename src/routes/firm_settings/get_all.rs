use actix_web::{web, HttpResponse, Responder};

use crate::external_services_client::{ServicesClient, ServicesEnum};

pub async fn get_firm_settings(services_client: web::Data<ServicesClient>) -> impl Responder {
    let response = services_client
        .get(ServicesEnum::Locator, "/api/settings/firm", &vec![])
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
