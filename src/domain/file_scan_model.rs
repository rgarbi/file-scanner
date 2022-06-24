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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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

    pub fn from_str(val: &str) -> ScanStatus {
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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;
    use crate::domain::file_scan_model::{FileScan, ScanStatus};

    #[test]
    fn scan_status_to_and_from_str_test() {
        assert_eq!(ScanStatus::from_str("Pending").as_str(), ScanStatus::Pending.as_str());
        assert_eq!(ScanStatus::from_str("Error").as_str(), ScanStatus::Error.as_str());
        assert_eq!(ScanStatus::from_str("Scanning").as_str(), ScanStatus::Scanning.as_str());
        assert_eq!(ScanStatus::from_str("Hashing").as_str(), ScanStatus::Hashing.as_str());
        assert_eq!(ScanStatus::from_str("DoneBadFile").as_str(), ScanStatus::DoneBadFile.as_str());
        assert_eq!(ScanStatus::from_str("DoneClean").as_str(), ScanStatus::DoneClean.as_str());
    }

    #[test]
    fn file_scan_to_json_works() {
        let file_scan = FileScan {
            id: Uuid::new_v4(),
            file_name: Uuid::new_v4().to_string(),
            file_location: Uuid::new_v4().to_string(),
            file_hash: Uuid::new_v4().to_string(),
            posted_on: Utc::now(),
            last_updated: Utc::now(),
            status: ScanStatus::Pending
        };
        let _json = file_scan.to_json();
    }
}