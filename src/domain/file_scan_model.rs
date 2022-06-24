use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileScan {
    pub id: Uuid,
    pub file_name: String,
    pub file_location: String,
    pub file_hash: String,
    pub posted_on: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: ScanStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ScanStatus {
    Pending,
    Hashing,
    Scanning,
    Error,
    DoneClean,
    DoneBadFile,
}

impl ScanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScanStatus::Pending => "Pending",
            ScanStatus::Hashing => "Hashing",
            ScanStatus::Scanning => "Scanning",
            ScanStatus::Error => "Error",
            ScanStatus::DoneClean => "DoneClean",
            ScanStatus::DoneBadFile => "DoneBadFile",
        }
    }

    pub fn from_str(val: String) -> ScanStatus {
        if val.eq("Pending") {
            return ScanStatus::Pending;
        }

        if val.eq("Hashing") {
            return ScanStatus::Hashing;
        }

        if val.eq("Scanning") {
            return ScanStatus::Scanning;
        }

        if val.eq("Error") {
            return ScanStatus::Error;
        }

        if val.eq("DoneClean") {
            return ScanStatus::DoneClean;
        }

        if val.eq("DoneBadFile") {
            return ScanStatus::DoneBadFile;
        }

        error!("Could not map string: {} to the enum SubscriptionType", val);
        ScanStatus::Error
    }

}

impl FileScan {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Was not able to serialize.")
    }
}