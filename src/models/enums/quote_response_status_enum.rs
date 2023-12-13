use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Copy, Debug)]
pub enum QuoteResponseStatusEnum {
    Cancelled = 0,
    Expired = 1,
    Failed = 2,
    RejectedBadRequest = 3,
    RejectedDuplicate = 4,
    WaitingAcceptance = 5,
    Partial = 6,
    Filled = 7,
    NoInventory = 8,
    AutoAccepted = 9,
    AutoRejected = 10,

    RequestAccepted = 11, //Do we need to acknowledge sender that we accepted request?
    RejectedProhibited = 12,
}
