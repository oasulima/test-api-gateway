use crate::external_services_client::{ServicesClient, ServicesEnum};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryInfo {
    symbol: String,
}

pub async fn get_internal_inventory(
    services_client: web::Data<ServicesClient>,
    query_info: web::Query<QueryInfo>,
) -> impl Responder {
    let query_params: Vec<(&str, String)> = vec![("symbol", query_info.symbol.clone())];

    let response = services_client
        .get(
            ServicesEnum::InternalInventory,
            "/api/admin/GetInventory",
            &query_params,
        )
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
