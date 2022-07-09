use std::process::Stdio;
use sqlx::PgPool;
use crate::db::file_scan_broker::select_a_file_that_needs_worked;
use crate::domain::file_scan_model::{FileScan, ScanResult, ScanStatus};
use tokio::process::Command;
use tracing::Level;

pub static MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN: i64 = 10;

pub async fn scan_files(pg_pool: &PgPool) {
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
               scan_file(&maybe_a_file_scan.unwrap()).await
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

pub async fn scan_file(file_scan: &FileScan) -> Result<ScanResult, Error> {
    let command = Command::new("ls").stdout(Stdio::null());

    let child_process_handle = command.spawn();

    match child_process_handle {
        Ok(mut child) =>{
            let result = child.wait().await;

            match result {
                Ok(exit_status) => {
                    if exit_status.exit_ok().is_ok() {
                        return Ok(ScanResult::Clean);
                    }

                    return Err(exit_status.)
                }
            }

        },
        Err(error) => {
            return Err(error);
        }
    }

    Ok(ScanResult::Clean)
}
