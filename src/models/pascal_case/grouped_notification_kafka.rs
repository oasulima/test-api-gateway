use serde::Deserialize;
use time::OffsetDateTime;

use crate::models::NotificationType;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GroupedNotificationKafka {
    pub r#type: NotificationType,
    pub kind: String,
    pub group_parameters: String,
    pub last_message: String,
    #[serde(with = "time::serde::iso8601")]
    pub first_time: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub last_time: OffsetDateTime,
    pub count: i32,
}
