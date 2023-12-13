use actix_web::Responder;
use actix_web::{web, HttpResponse};

use crate::external_services_client::{ServicesClient, ServicesEnum};

pub async fn get_default_symbol_setting(
    services_client: web::Data<ServicesClient>,
) -> impl Responder {
    //
    let response = services_client
        .get(ServicesEnum::Locator, "/api/settings/default/symbol", &vec![])
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
