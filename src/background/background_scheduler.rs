use crate::background::background_hasher::hash_files;
use crate::background::background_scanner::scan_files;
use sqlx::PgPool;
use std::time::Duration;
use tokio::{task, time};

pub const HASHING_SCHEDULE: u64 = 500;
pub const SCANNING_SCHEDULE: u64 = 1000;
pub const REMOVAL_SCHEDULE: u64 = 10000;

pub async fn spin_up_background_tasks(pg_pool: PgPool) {
    schedule_hashing(pg_pool.clone()).await;
    schedule_file_scanning(pg_pool.clone()).await;
    schedule_file_removal(pg_pool.clone()).await;
}

pub async fn schedule_hashing(pg_pool: PgPool) {
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(HASHING_SCHEDULE));

        loop {
            interval.tick().await;
            let a_pool = pg_pool.clone();
            let handle = task::spawn(async move {
                hash_files(&a_pool).await;
            });
            handle.await.unwrap();
        }
    });
}

pub async fn schedule_file_scanning(pg_pool: PgPool) {
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(SCANNING_SCHEDULE));

        loop {
            interval.tick().await;
            let a_pool = pg_pool.clone();
            let handle = task::spawn(async move {
                scan_files(&a_pool).await;
            });
            handle.await.unwrap();
        }
    });
}

pub async fn schedule_file_removal(pg_pool: PgPool) {
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(REMOVAL_SCHEDULE));

        loop {
            interval.tick().await;
            let a_pool = pg_pool.clone();
            let handle = task::spawn(async move {
                scan_files(&a_pool).await;
            });
            handle.await.unwrap();
        }
    });
}
