use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum NotificationType {
    Warning,
    Error,
    Critical,
}
