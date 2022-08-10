use crate::helper::{generate_queue_item, spawn_app};
use claim::{assert_err, assert_ok};
use db::queue_item_broker::store;
use file_scanner::db;

#[tokio::test]
async fn insert_scan_works() {
    let app = spawn_app(false).await;

    let queue_item = generate_queue_item();
    assert_ok!(store(queue_item, &app.db_pool).await);
}

#[tokio::test]
async fn insert_scan_fails() {
    let app = spawn_app(false).await;
    let queue_item = generate_queue_item();

    // Sabotage the database
    sqlx::query!("ALTER TABLE queue_items DROP COLUMN queue_item_contents;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    assert_err!(store(queue_item, &app.db_pool).await);
}
