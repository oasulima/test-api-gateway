use crate::external_services_client::{ServicesClient, ServicesEnum};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultSymbolSettingRequest {
    #[serde(rename = "minUserPrice")]
    min_user_price: Option<f64>,
    vig: Option<f64>,
    multiplier: Option<f64>,
    #[serde(rename = "oneTimeLocateDiscount")]
    one_time_locate_discount: Option<f64>,
    #[serde(rename = "oneTimePreBorrowDiscount")]
    one_time_pre_borrow_discount: Option<f64>,
}

pub async fn update_default_symbol_setting(
    services_client: web::Data<ServicesClient>,
    data: web::Json<DefaultSymbolSettingRequest>,
) -> impl Responder {
    let response = services_client
        .put(ServicesEnum::Locator, "/api/settings/default/symbol", &data)
        .await
        .unwrap();

    HttpResponse::Ok().streaming(response.bytes_stream())
}
