use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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
    pub being_worked: bool,
    pub work_started: Option<i64>,
    pub scan_result: Option<ScanResult>,
    pub scan_result_details: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ScanStatus {
    Pending,
    Hashing,
    DoneHashing,
    Scanning,
    DoneScanningClean,
    DoneScanningBadFile,
    Error,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ScanResult {
    Clean,
    BadFile,
}

impl FromStr for ScanResult {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        if val.eq("Clean") {
            return Ok(ScanResult::Clean);
        }

        if val.eq("BadFile") {
            return Ok(ScanResult::BadFile);
        }

        error!("Could not map string: {} to the enum SubscriptionType", val);
        Err(())
    }
}

impl ScanResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScanResult::Clean => "Clean",
            ScanResult::BadFile => "BadFile",
        }
    }

    pub fn from_optional_string(value: Option<String>) -> Option<ScanResult> {
        if let Some(..) = value {
            let result = ScanResult::from_str(value.unwrap().as_str());
            return match result {
                Ok(scan_result) => Some(scan_result),
                Err(_) => None,
            };
        }

        None
    }

    pub fn to_optional_string(value: Option<ScanResult>) -> Option<String> {
        if let Some(..) = value {
            return Some(String::from(value.unwrap().as_str()));
        }

        None
    }
}

impl FromStr for ScanStatus {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        if val.eq("Pending") {
            return Ok(ScanStatus::Pending);
        }

        if val.eq("Hashing") {
            return Ok(ScanStatus::Hashing);
        }

        if val.eq("DoneHashing") {
            return Ok(ScanStatus::DoneHashing);
        }

        if val.eq("Scanning") {
            return Ok(ScanStatus::Scanning);
        }

        if val.eq("Error") {
            return Ok(ScanStatus::Error);
        }

        if val.eq("DoneScanningClean") {
            return Ok(ScanStatus::DoneScanningClean);
        }

        if val.eq("DoneScanningBadFile") {
            return Ok(ScanStatus::DoneScanningBadFile);
        }

        error!("Could not map string: {} to the enum SubscriptionType", val);
        Err(())
    }
}

impl ScanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScanStatus::Pending => "Pending",
            ScanStatus::Hashing => "Hashing",
            ScanStatus::DoneHashing => "DoneHashing",
            ScanStatus::Scanning => "Scanning",
            ScanStatus::Error => "Error",
            ScanStatus::DoneScanningClean => "DoneScanningClean",
            ScanStatus::DoneScanningBadFile => "DoneScanningBadFile",
        }
    }
}

impl FileScan {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Was not able to serialize.")
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::file_scan_model::{FileScan, ScanResult, ScanStatus};
    use chrono::Utc;
    use claim::{assert_err, assert_none, assert_some};
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn scan_status_to_and_from_str_test() {
        assert_eq!(
            ScanStatus::from_str("Pending").unwrap().as_str(),
            ScanStatus::Pending.as_str()
        );
        assert_eq!(
            ScanStatus::from_str("Error").unwrap().as_str(),
            ScanStatus::Error.as_str()
        );
        assert_eq!(
            ScanStatus::from_str("Scanning").unwrap().as_str(),
            ScanStatus::Scanning.as_str()
        );
        assert_eq!(
            ScanStatus::from_str("Hashing").unwrap().as_str(),
            ScanStatus::Hashing.as_str()
        );
        assert_eq!(
            ScanStatus::from_str("DoneScanningBadFile")
                .unwrap()
                .as_str(),
            ScanStatus::DoneScanningBadFile.as_str()
        );
        assert_eq!(
            ScanStatus::from_str("DoneScanningClean").unwrap().as_str(),
            ScanStatus::DoneScanningClean.as_str()
        );

        assert_err!(ScanStatus::from_str(Uuid::new_v4().to_string().as_str()));
    }

    #[test]
    fn scan_result_to_and_from_str_test() {
        assert_eq!(
            ScanResult::from_str("Clean").unwrap().as_str(),
            ScanResult::Clean.as_str()
        );
        assert_eq!(
            ScanResult::from_str("BadFile").unwrap().as_str(),
            ScanResult::BadFile.as_str()
        );

        assert_err!(ScanResult::from_str(Uuid::new_v4().to_string().as_str()));
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
            status: ScanStatus::Pending,
            being_worked: false,
            work_started: Some(0),
            scan_result: None,
            scan_result_details: None,
        };
        let _json = file_scan.to_json();
    }

    #[test]
    fn scan_result_to_from_optional() {
        assert_some!(ScanResult::from_optional_string(Some(String::from(
            ScanResult::Clean.as_str()
        ))));
        assert_some!(ScanResult::from_optional_string(Some(String::from(
            ScanResult::BadFile.as_str()
        ))));
        assert_none!(ScanResult::from_optional_string(None));
        assert_none!(ScanResult::from_optional_string(Some(
            Uuid::new_v4().to_string()
        )));

        assert_some!(ScanResult::to_optional_string(Some(ScanResult::Clean)));
        assert_some!(ScanResult::to_optional_string(Some(ScanResult::BadFile)));
        assert_none!(ScanResult::to_optional_string(None));
    }
}
