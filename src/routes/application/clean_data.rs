use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;

use crate::external_services_client::{ServicesClient, ServicesEnum};

#[derive(thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub async fn clean_all_services_data(
    services_client: web::Data<ServicesClient>,
) -> Result<HttpResponse, AppError> {
    let clean_locator_data_response = clean_locator_data(&services_client);

    let clean_internal_inventory_data_response = clean_internal_inventory_data(&services_client);

    let clean_order_book_data = clean_order_book_data(&services_client);

    let (
        clean_locator_data_response,
        clean_internal_inventory_data_response,
        clean_order_book_data,
    ) = tokio::join!(
        clean_locator_data_response,
        clean_internal_inventory_data_response,
        clean_order_book_data
    );

    clean_locator_data_response.context("Failed to call Locator")?;
    clean_internal_inventory_data_response.context("Failed to call Internal Inventory")?;
    clean_order_book_data.context("Failed to call Orderbook")?;

    clean_admin_ui_data();

    Ok(HttpResponse::Ok().body("true"))
}

fn clean_admin_ui_data() {
    // todo!("clean collections of locates and locate history")
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

async fn clean_internal_inventory_data(
    services_client: &ServicesClient,
) -> Result<(), reqwest::Error> {
    services_client
        .post(
            ServicesEnum::InternalInventory,
            "/api/application/cleanData",
            &vec![],
            &(),
        )
        .await?;
    Ok(())
}

async fn clean_order_book_data(services_client: &ServicesClient) -> Result<(), reqwest::Error> {
    services_client
        .post(
            ServicesEnum::OrderBook,
            "/api/application/cleanData",
            &vec![],
            &(),
        )
        .await?;
    Ok(())
}

async fn clean_locator_data(services_client: &ServicesClient) -> Result<(), reqwest::Error> {
    services_client
        .post(
            ServicesEnum::Locator,
            "/api/application/cleanData",
            &vec![],
            &(),
        )
        .await?;
    Ok(())
}
