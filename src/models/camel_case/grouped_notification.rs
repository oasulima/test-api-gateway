use serde::Serialize;
use time::OffsetDateTime;

use crate::models::{GroupedNotificationKafka, NotificationType};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupedNotification {
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

impl From<GroupedNotificationKafka> for GroupedNotification {
    fn from(value: GroupedNotificationKafka) -> Self {
        Self {
            r#type: value.r#type,
            kind: value.kind,
            group_parameters: value.group_parameters,
            last_message: value.last_message,
            first_time: value.first_time,
            last_time: value.last_time,
            count: value.count,
        }
    }
}
