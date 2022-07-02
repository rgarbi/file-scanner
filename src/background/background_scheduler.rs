use std::time::Duration;
use sqlx::PgPool;
use tokio::{task, time};
use crate::background::background_hasher::hash_files;


pub async fn spin_up_background_tasks(pg_pool: PgPool) {
    schedule_hashing(pg_pool).await;
}

pub async fn schedule_hashing(pg_pool: PgPool) {
    let _handle = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(500));

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