use crate::{
    external_services_client::{ServicesClient, ServicesEnum},
    models::FirmProviderSettingRequest,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize)]
pub struct QueryInfo {
    firm_name: String,
}

pub async fn replace_firm_provider_settings(
    services_client: web::Data<ServicesClient>,
    query_info: web::Path<QueryInfo>,
    data: web::Json<Vec<FirmProviderSettingRequest>>,
) -> impl Responder {
    let response = services_client
        .post(
            ServicesEnum::Locator,
            &format!(
                "/api/settings/firm/{}/provider",
                encode(&query_info.firm_name)
            ),
            &vec![],
            &data,
        )
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
