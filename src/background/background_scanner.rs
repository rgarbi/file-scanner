use sqlx::PgPool;
use crate::db::file_scan_broker::select_a_file_that_needs_worked;
use crate::domain::file_scan_model::{FileScan, ScanStatus};

pub static MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN: i64 = 10;
pub async fn scan_files(_pg_pool: &PgPool) {
    //get a lock on a file that has been hashed but has not been scanned.
    let get_file_result = select_a_file_that_needs_worked(
        ScanStatus::DoneHashing,
        ScanStatus::Scanning,
        MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN,
        pg_pool,
    )
        .await;

    match get_file_result {
        Ok(maybe_a_file_scan) => {
            if maybe_a_file_scan.is_some() {
                let file_scan = maybe_a_file_scan.unwrap();
                //hash the file.
                let file_hash = hash_a_file(file_scan.file_location.clone()).await;
                //update the record
                let _save_result =
                    set_a_file_scan_to_be_done_hashing(file_scan.id, file_hash, pg_pool).await;
            }
        }
        Err(err) => {
            tracing::event!(Level::ERROR, "Err: {:?}", err);
        }
    }
    todo!("We need to handle errors and retries... Also... we need to handle when we are done scanning and it was a bad file!")
    //scan the file.
    //update the record
}

pub async fn scan_file(file_scan: &FileScan) ->
