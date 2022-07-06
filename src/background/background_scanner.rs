use sqlx::PgPool;
use crate::db::file_scan_broker::select_a_file_that_needs_worked;
use crate::domain::file_scan_model::ScanStatus;

pub static MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN: i64 = 10;
pub async fn scan_files(_pg_pool: &PgPool) {
    //get a lock on a file that has been hashed but has not been scanned.
    let _get_file_result = select_a_file_that_needs_worked(
        ScanStatus::DoneHashing,
        ScanStatus::DoneScanningClean,
        MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN,
        pg_pool,
    )
        .await;

    todo!("We need to handle errors and retries... Also... we need to handle when we are done scanning and it was a bad file!")
    //scan the file.
    //update the record
}
