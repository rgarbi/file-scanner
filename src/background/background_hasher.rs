use crate::db::file_scan_broker::{
    select_a_file_that_needs_worked, set_a_file_scan_to_be_done_hashing,
};
use crate::domain::file_scan_model::ScanStatus;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use sqlx::PgPool;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::Level;

pub static MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN: i64 = 15;

pub async fn hash_files(pg_pool: &PgPool) {
    //get a lock on a file that has been posted but has not been hashed.
    let get_file_result = select_a_file_that_needs_worked(
        ScanStatus::Pending,
        ScanStatus::Hashing,
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
}

async fn hash_a_file(path: String) -> String {
    let input = File::open(path).await.unwrap();
    let digest = sha256_digest(input).await;
    HEXUPPER.encode(digest.as_ref())
}

async fn sha256_digest(mut file: File) -> Digest {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = file.read(&mut buffer).await.unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    context.finish()
}
