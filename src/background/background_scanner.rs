use std::process::Stdio;
use sqlx::PgPool;
use crate::db::file_scan_broker::select_a_file_that_needs_worked;
use crate::domain::file_scan_model::{FileScan, ScanResult, ScanStatus};
use tokio::process::Command;
use tracing::Level;

#[derive(Debug, Clone)]
pub struct ScanProcessError;

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
                let _result = scan_file(&maybe_a_file_scan.unwrap()).await;

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

pub async fn scan_file(_file_scan: &FileScan) -> Result<ScanResult, ScanProcessError> {
    let mut command = Command::new("ls");
    command.stdout(Stdio::null());

    let child_process_handle = command.spawn()
        .expect("ls command failed to start")
        .wait()
        .await
        .expect("ls command failed to run");

    if child_process_handle.success() {
        println!("command success: {}", child_process_handle);
        Ok(ScanResult::Clean)
    } else {
        println!("command error: {}", child_process_handle);
        Err(ScanProcessError)
    }
}
