use crate::{
    models::FirmSettingRequest,
    external_services_client::{ServicesClient, ServicesEnum},
};
use actix_web::{web, HttpResponse, Responder};

pub async fn update_firm_setting(
    services_client: web::Data<ServicesClient>,
    data: web::Json<FirmSettingRequest>,
) -> impl Responder {
    let response = services_client
        .put(ServicesEnum::Locator, "/api/settings/firm", &data)
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
