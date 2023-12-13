use super::{
    notification_service::NotificationsService, LocateRequestsCache, LocatesCache, TimeService,
};
use crate::models::QuoteResponseStatusEnum;
use crate::{
    external_services_client::{ServicesClient, ServicesEnum},
    models::{LocateModel, LocateRequestModel, LocatorQuoteResponse},
};
use actix_web::web;
use std::sync::Arc;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

const RESTORE_DATA_TAKE: i32 = 2_000;

pub struct MessageHandler {
    express_restoring_stop_cts: CancellationToken,
    restoring_data_task: Option<JoinHandle<()>>,
    locates_requests_cache: Arc<LocateRequestsCache>,
    locates_cache: Arc<LocatesCache>,
}

impl MessageHandler {
    pub fn new(
        notification_service: web::Data<NotificationsService>,

        time_service: web::Data<TimeService>,
        services_client: web::Data<ServicesClient>,
    ) -> Self {
        let locates_requests_cache = Arc::new(LocateRequestsCache::new());
        let locates_cache = Arc::new(LocatesCache::new());

        let express_restoring_stop_cts = CancellationToken::new();
        let cloned_express_restoring_stop_cts = express_restoring_stop_cts.clone();
        let mut x = Self {
            express_restoring_stop_cts,
            restoring_data_task: None,
            locates_requests_cache: locates_requests_cache.clone(),
            locates_cache: locates_cache.clone(),
        };

        x.restoring_data_task = Some(tokio::spawn(async {
            MessageHandler::restore_data(
                time_service,
                cloned_express_restoring_stop_cts,
                services_client,
                locates_requests_cache,
                locates_cache,
                notification_service,
            )
            .await;
        }));
        x
    }

    async fn restore_data(
        time_service: web::Data<TimeService>,
        express_restoring_stop_cts: CancellationToken,
        services_client: web::Data<ServicesClient>,
        locates_requests_cache: Arc<LocateRequestsCache>,
        locates_cache: Arc<LocatesCache>,
        notification_service: web::Data<NotificationsService>,
    ) {
        let (tx, mut rx) = mpsc::channel::<(Vec<LocateRequestModel>, Vec<LocateModel>)>(32);

        let utc_now = OffsetDateTime::now_utc();
        let from = time_service.get_previous_cleanup_time_in_utc(utc_now);

        let mut skip = 0;

        let cloned_express_restoring_stop_cts = express_restoring_stop_cts.clone();
        let cloned_notification_service = notification_service.clone();

        tokio::spawn(async move {
            while !express_restoring_stop_cts.is_cancelled() {
                let responses = get_locator_quote_responses(
                    &services_client,
                    from,
                    utc_now,
                    RESTORE_DATA_TAKE,
                    skip,
                )
                .await;

                if responses.is_empty() {
                    break;
                }

                let cloned_express_restoring_stop_cts = express_restoring_stop_cts.clone();
                let locates_requests_cache = locates_requests_cache.clone();
                let locates_cache = locates_cache.clone();
                handle_responses(
                    cloned_express_restoring_stop_cts,
                    responses,
                    tx.clone(),
                    locates_requests_cache,
                    locates_cache,
                )
                .await;

                skip +=  RESTORE_DATA_TAKE;
            }
        });

        send_handled_resposes(
            cloned_notification_service,
            cloned_express_restoring_stop_cts,
            &mut rx,
        )
        .await;
    }

    pub fn get_locate_requests_history(&self) -> Vec<LocateRequestModel> {
        self.locates_requests_cache.get_history_records()
    }

    pub fn get_locates_history(&self) -> Vec<LocateModel> {
        self.locates_cache.get_history_records()
    }

    pub async fn handler_quote_response(
        &self,
        quote_response: LocatorQuoteResponse,
        notification_service: web::Data<NotificationsService>,
    ) {
        let (locate_request, locate) = handle(
            quote_response,
            self.locates_requests_cache.clone(),
            self.locates_cache.clone(),
        );

        if let Some(locate_request) = locate_request {
            notification_service
                .send_locate_request(locate_request)
                .await;
        }

        if let Some(locate) = locate {
            notification_service.send_locate(locate).await;
        }
    }
}

async fn handle_responses(
    express_restoring_stop_cts: CancellationToken,
    responses: Vec<LocatorQuoteResponse>,
    handled_responses_to_send: tokio::sync::mpsc::Sender<(
        Vec<LocateRequestModel>,
        Vec<LocateModel>,
    )>,
    locates_requests_cache: Arc<LocateRequestsCache>,
    locates_cache: Arc<LocatesCache>,
) {
    let mut locate_requests: Vec<LocateRequestModel> = Vec::new();
    let mut locates: Vec<LocateModel> = Vec::new();

    //let mut handled_responses_to_send = handled_responses_to_send.lock().unwrap();

    for response in responses {
        if express_restoring_stop_cts.is_cancelled() {
            break;
        }
        let locates_requests_cache = locates_requests_cache.clone();
        let locates_cache = locates_cache.clone();
        let (locate_request, locate) = handle(response, locates_requests_cache, locates_cache);

        if let Some(locate_request) = locate_request {
            locate_requests.push(locate_request);
        }

        if let Some(locate) = locate {
            locates.push(locate);
        }
    }

    if !express_restoring_stop_cts.is_cancelled() {
        let result = handled_responses_to_send
            .send((locate_requests, locates))
            .await;
        match result {
            Ok(value) => tracing::info!("message sent to channel: {:?}", value),
            Err(err) => tracing::error!("couldn't send messages to channel: {:?}", err),
        }
    }
}

fn handle(
    response: LocatorQuoteResponse,
    locates_requests_cache: Arc<LocateRequestsCache>,
    locates_cache: Arc<LocatesCache>,
) -> (Option<LocateRequestModel>, Option<LocateModel>) {
    let response_status = response.status;

    let locate_request = match response_status {
        QuoteResponseStatusEnum::WaitingAcceptance
        | QuoteResponseStatusEnum::AutoAccepted
        | QuoteResponseStatusEnum::AutoRejected
        | QuoteResponseStatusEnum::RejectedDuplicate
        | QuoteResponseStatusEnum::RejectedBadRequest
        | QuoteResponseStatusEnum::NoInventory => {
            Some(locates_requests_cache.memorize(response.clone()))
        }

        _ => None,
    };

    let locate = match response_status {
        QuoteResponseStatusEnum::Cancelled
        | QuoteResponseStatusEnum::Expired
        | QuoteResponseStatusEnum::Failed
        | QuoteResponseStatusEnum::RejectedBadRequest
        | QuoteResponseStatusEnum::RejectedDuplicate
        | QuoteResponseStatusEnum::Partial
        | QuoteResponseStatusEnum::Filled
        | QuoteResponseStatusEnum::NoInventory
        | QuoteResponseStatusEnum::AutoRejected => Some(locates_cache.memorize(response.clone())),

        _ => None,
    };

    (locate_request, locate)
}

async fn send_handled_resposes(
    notifications_service: web::Data<NotificationsService>,
    express_restoring_stop_cts: CancellationToken,
    rx: &mut tokio::sync::mpsc::Receiver<(Vec<LocateRequestModel>, Vec<LocateModel>)>,
) {
    while let Some((locate_requests, locates)) = rx.recv().await {
        if !express_restoring_stop_cts.is_cancelled() && !locate_requests.is_empty() {
            notifications_service
                .send_locate_requests_history_to_clients(&locate_requests)
                .await;
        }

        if !express_restoring_stop_cts.is_cancelled() && !locates.is_empty() {
            notifications_service
                .send_locates_history_to_clients(&locates)
                .await;
        }
    }
}

async fn get_locator_quote_responses(
    services_client: &ServicesClient,
    from: OffsetDateTime,
    to: OffsetDateTime,
    take: i32,
    skip: i32,
) -> Vec<LocatorQuoteResponse> {
    let query_params: Vec<(&str, String)> = vec![
        ("from", from.format(&Iso8601::DEFAULT).unwrap()),
        ("to", to.format(&Iso8601::DEFAULT).unwrap()),
        ("take", take.to_string()),
        ("skip", skip.to_string()),
    ];

    let response = services_client
        .get(
            ServicesEnum::Reporting,
            "/api/report/data/quote/responses",
            &query_params,
        )
        .await
        .unwrap();

    let status = response.status();
    let text = response.text().await.unwrap();

    // let full = response.bytes().await.unwrap();

    // serde_json::from_slice(&full)

    let jd = &mut serde_json::Deserializer::from_str(&text);

    let result: Result<Vec<LocatorQuoteResponse>, _> = serde_path_to_error::deserialize(jd);

    match result {
        Ok(value) => value,
        Err(err) => {
            tracing::error!("Couldn't parse LocatorQuoteResponse: {:?}", err);
            tracing::error!("status: {:?}", status);
            tracing::error!("text: {}", text);
            vec![]
        }
    }

    //vec![]
}

/*
[{"id":"5107ff202220_1231007140903774000","firmId":"admin","accountId":"osulima","symbol":"AAPL","status":"Filled","time":"2023-10-07T14:09:06.173","errorMessage":null,"reqQty":1000,"fillQty":1000,"price":0.10000,"tzPrice":0.10000,"source":"FPMOCK","sources":[{"provider":"fpmock","source":"FPMOCK","price":0.1,"qty":1000,"userPrice":0.1000,"discountedPrice":0.100000}]},{"id":"5107ff202220_1231007140903774000","firmId":"admin","accountId":"osulima","symbol":"AAPL","status":"AutoAccepted","time":"2023-10-07T14:09:04.863","errorMessage":null,"reqQty":1000,"fillQty":1000,"price":0.10000,"tzPrice":0.10000,"source":"FPMOCK","sources":[{"provider":"FPMOCK","source":"FPMOCK","price":0.1,"qty":1000,"userPrice":2.1000000000000000,"discountedPrice":0.100000}]},{"id":"5107ff202220_1231007140903774000","firmId":"admin","accountId":"osulima","symbol":"AAPL","status":"RequestAccepted","time":"2023-10-07T14:09:04.817","errorMessage":null,"reqQty":1000,"fillQty":0,"price":0.00000,"tzPrice":0.00000,"source":"","sources":[]}]

*/

#[cfg(test)]
mod tests {

    use serde::Deserialize;

    use crate::models::{LocatorQuoteResponse, QuoteResponseStatusEnum, QuoteSourceInfo};

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocatorQuoteResponseNew {
        pub id: String,
        pub firm_id: String,
        pub account_id: String,
        pub symbol: String,

        pub status: QuoteResponseStatusEnum,
        //#[serde(with = "time::serde::rfc3339")]
        pub time: String,
        pub error_message: Option<String>,
        pub req_qty: i32,
        pub fill_qty: Option<i32>,
        pub price: Option<f64>,
        pub tz_price: f64,
        pub source: String,
        pub sources: Vec<QuoteSourceInfo>,
    }

    #[tokio::test]
    async fn deserialize_time() {
        // Arrange
        let text = r#"[{"id":"5107ff202220_1231007140903774000","firmId":"admin","accountId":"osulima","symbol":"AAPL","status":"Filled","time":"2023-10-07T14:09:06.173Z","errorMessage":null,"reqQty":1000,"fillQty":1000,"price":0.10000,"tzPrice":0.10000,"source":"FPMOCK","sources":[{"provider":"fpmock","source":"FPMOCK","price":0.1,"qty":1000,"userPrice":0.1000,"discountedPrice":0.100000}]}]    "#;
        // Act
        let jd = &mut serde_json::Deserializer::from_str(&text);

        let result: Result<Vec<LocatorQuoteResponse>, _> = serde_path_to_error::deserialize(jd);

        // Assert
        match result {
            Ok(value) => {
                let x = value[0].clone();

                assert_eq!(x.id, "5107ff202220_1231007140903774000");
                // let y: OffsetDateTime =OffsetDateTime::parse(x.time, description)  x.time
            }
            Err(err) => {
                tracing::info!("Couldn't parse LocatorQuoteResponse: {:?}", err);
                tracing::error!("text: {}", text);
                let path = err.path().to_string();
                assert_eq!(path, "dependencies.serde.version");
            }
        };
    }
}
