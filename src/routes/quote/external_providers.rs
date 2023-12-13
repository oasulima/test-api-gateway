use crate::external_services_client::{ServicesClient, ServicesEnum};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteExternalProvidersRequest {
    symbol: String,
}

pub async fn quote_external_providers(
    services_client: web::Data<ServicesClient>,
    data: web::Json<QuoteExternalProvidersRequest>,
) -> impl Responder {
    let query_params: Vec<(&str, String)> = vec![("symbol", data.symbol.clone())];

    let response = services_client
        .post(
            ServicesEnum::Locator,
            "/api/locator/trigger-pinger",
            &query_params,
            &(),
        )
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
