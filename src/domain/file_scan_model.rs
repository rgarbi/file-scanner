use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct FileScan {
    pub id: Uuid,
    pub file_name: String,
    pub file_location: String,
    pub file_hash: String,
    pub posted_on: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: ScanStatus,
}

pub enum ScanStatus {
    Pending,
    Scanning,
    Error,
    DoneClean,
    DoneBadFile
}

impl FileScan {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Was not able to serialize.")
    }
}