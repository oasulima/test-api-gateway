use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::models::{LocateRequestModel, LocatorQuoteResponse};

pub struct LocateRequestsCache {
    locate_requests_cache: Arc<Mutex<HashMap<String, LocateRequestModel>>>,
}

impl LocateRequestsCache {
    pub fn new() -> Self {
        Self {
            locate_requests_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_history_records(&self) -> Vec<LocateRequestModel> {
        let locate_requests_cache = self.locate_requests_cache.lock().unwrap();
        let mut values: Vec<LocateRequestModel> =
            locate_requests_cache.iter().map(|x| x.1.clone()).collect();
        values.sort_by(|a, b| b.time.cmp(&a.time));
        values.clone()
    }

    pub fn memorize(&self, message: LocatorQuoteResponse) -> LocateRequestModel {
        let sources = message.sources;
        let id = message.id.clone();
        let mut new_entity = LocateRequestModel {
            id: id.to_string(),
            account_id: message.account_id,
            firm_id: message.firm_id,
            symbol: message.symbol,
            qty_req: message.req_qty,
            qty_offer: message.fill_qty.unwrap_or(0),
            price: message.price.unwrap_or(0.0),
            tz_price: message.tz_price,
            source: sources
                .iter()
                .map(|x| x.source.clone())
                .collect::<Vec<String>>()
                .join(", "),
            source_details: sources,
            time: message.time,
        };

        let mut locate_requests_cache = self.locate_requests_cache.lock().unwrap();

        let entity = locate_requests_cache.get(&id);

        if let Some(entity) = entity {
            new_entity.time = entity.time;
        }

        locate_requests_cache.insert(id, new_entity.clone());

        new_entity
    }
}
