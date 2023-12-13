use crate::{
    external_services_client::{ServicesClient, ServicesEnum},
    models::FirmSettingRequest,
};
use actix_web::{web, HttpResponse, Responder};

pub async fn add_firm_setting(
    services_client: web::Data<ServicesClient>,
    data: web::Json<FirmSettingRequest>,
) -> impl Responder {
    let response = services_client
        .post(ServicesEnum::Locator, "/api/settings/firm", &vec![], &data)
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
