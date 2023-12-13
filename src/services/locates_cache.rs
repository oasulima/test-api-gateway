use std::sync::{Arc, Mutex};

use crate::models::{LocateModel, LocatorQuoteResponse};

pub struct LocatesCache {
    locates_cache: Arc<Mutex<Vec<LocateModel>>>,
}

impl LocatesCache {
    pub fn new() -> Self {
        Self {
            locates_cache: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_history_records(&self) -> Vec<LocateModel> {
        let mut locates_cache = self.locates_cache.lock().unwrap();
        locates_cache.sort_by(|a, b| b.time.cmp(&a.time));
        locates_cache.clone()
    }

    pub fn memorize(&self, message: LocatorQuoteResponse) -> LocateModel {
        let entity = LocateModel {
            quote_id: message.id,
            time: message.time,
            account_id: message.account_id,
            firm_id: message.firm_id,
            symbol: message.symbol,
            qty_fill: message.fill_qty.unwrap_or(0),
            req_qty: message.req_qty,
            user_price: message.price.unwrap_or(0.0),
            tz_price: message.tz_price,
            status: message.status,
            error_message: message.error_message,
            source: message.source,
            source_details: message.sources,
        };

        let mut locates_cache = self.locates_cache.lock().unwrap();
        locates_cache.push(entity.clone());

        entity
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};

    use time::OffsetDateTime;

    use crate::{models::QuoteSourceInfo, services::LocatesCache};

    fn memorize_10(locates_cache: Arc<LocatesCache>) {
        for i in 1..=10 {
            locates_cache.memorize(crate::models::LocatorQuoteResponse {
                id: i.to_string(),
                firm_id: "fake_provider".to_string(),
                account_id: "oleg".to_string(),
                symbol: "AAPL".to_string(),
                status: crate::models::QuoteResponseStatusEnum::AutoAccepted,
                time: OffsetDateTime::now_utc(),
                error_message: None,
                req_qty: 1000,
                fill_qty: Some(1000),
                price: Some(1.0),
                tz_price: 1.0,
                source: "FPMOCK".to_string(),
                sources: vec![QuoteSourceInfo {
                    provider: "admin".to_string(),
                    source: "FPMOCK".to_string(),
                    price: 1.0,
                    qty: 1000,
                    user_price: 1000.0,
                    discounted_price: 1000.0,
                }],
            });
        }
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        // Arrange
        let locates_cache = Arc::new(LocatesCache::new());

        // Act
        let locates_cache1 = Arc::clone(&locates_cache);
        let task1 = thread::spawn(|| {
            memorize_10(locates_cache1);
        });

        let locates_cache2 = Arc::clone(&locates_cache);
        let task2 = thread::spawn(|| {
            memorize_10(locates_cache2);
        });

        let locates_cache3 = Arc::clone(&locates_cache);
        let task3 = thread::spawn(|| {
            memorize_10(locates_cache3);
        });

        let _ = task1.join();
        let _ = task2.join();
        let _ = task3.join();

        // Assert

        let x = locates_cache.get_history_records();
        assert_eq!(x.len(), 30);
    }
}
