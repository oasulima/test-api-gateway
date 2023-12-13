pub mod sse_events {
    pub const EXTERNAL_PROVIDER: &str = "external-provider";
    pub const LOCATE_REQUEST: &str = "locate-request";
    pub const LOCATE_REQUEST_HISTORY: &str = "locate-request-history";
    pub const LOCATE: &str = "locate";
    pub const LOCATE_HISTORY: &str = "locate-history";
    pub const NOTIFICATION: &str = "notification";
    pub const INTERNAL_INVENTORY: &str = "internal-inventory";
}

pub mod known_roles {

    pub const ADMIN: &str = "Locator.Admin";
    pub const VIEWER: &str = "Locator.View";
    pub const PROVIDER: &str = "Broker.TZC";
}

pub const SSE_DATA_CHUNK_SIZE: usize = 2_000;

pub mod providers_ids {
    pub const INTERNAL_INVENTORY: &str = "II";
    pub const ORDER_BOOK: &str = "ORBK";
}
