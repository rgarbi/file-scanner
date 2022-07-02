use std::time::Duration;
use sqlx::PgPool;
use tokio::{runtime, time};
use crate::background::background_hasher::hash_files;


pub async fn spin_up_background_tasks(pg_pool: &PgPool) {
    schedule_hashing(pg_pool).await;
}

pub async fn schedule_hashing(pg_pool: &PgPool) {
    let pool = pg_pool.clone();
    let threaded_rt = runtime::Runtime::new().unwrap();
    let _handle = threaded_rt.spawn(async move {
        let mut interval = time::interval(Duration::from_millis(250));

        loop {
            interval.tick().await;
            hash_files(&pool).await;
        }
    });
}