use crate::{
    external_services_client::{ServicesClient, ServicesEnum},
    services::TimeService,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use time::{format_description::well_known::Iso8601, OffsetDateTime};

#[derive(Deserialize)]
pub struct QueryInfo {
    symbol: String,
}

pub async fn get_internal_inventory_history(
    services_client: web::Data<ServicesClient>,
    time_service: web::Data<TimeService>,
    query_info: web::Query<QueryInfo>,
) -> impl Responder {
    let utc_now = OffsetDateTime::now_utc();
    let from = time_service.get_previous_cleanup_time_in_utc(utc_now);

    let query_params: Vec<(&str, String)> = vec![
        ("take", 10.to_string()),
        ("symbol", query_info.symbol.clone()),
        //("providerId",providerId),
        ("beforeCreatedAt", from.format(&Iso8601::DEFAULT).unwrap()),
    ];

    let response = services_client
        .get(
            ServicesEnum::Reporting,
            "/api/internal-inventory/items/history",
            &query_params,
        )
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
