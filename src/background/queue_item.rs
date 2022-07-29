use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueueItem {
    pub id: Uuid,
    pub queue_item_type: String,
    pub queue_item_contents: String,
    pub work_started: Option<i64>,
    pub being_worked: bool,
    pub error_count: i64,
    pub error_message: Option<String>,
}


