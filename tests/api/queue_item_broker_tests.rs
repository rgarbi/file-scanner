use crate::helper::{generate_queue_item, spawn_app};
use claim::{assert_err, assert_ok, assert_some};
use db::queue_item_broker::store;
use file_scanner::db;
use file_scanner::db::queue_item_broker::get_item_that_needs_worked;

#[tokio::test]
async fn insert_queue_item_works() {
    let app = spawn_app(false).await;

    let queue_item = generate_queue_item();
    assert_ok!(store(queue_item, &app.db_pool).await);
}

#[tokio::test]
async fn insert_queue_item_fails() {
    let app = spawn_app(false).await;
    let queue_item = generate_queue_item();

    // Sabotage the database
    sqlx::query!("ALTER TABLE queue_items DROP COLUMN queue_item_contents;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    assert_err!(store(queue_item, &app.db_pool).await);
}

#[tokio::test]
async fn get_queue_item_works() {
    let app = spawn_app(false).await;

    let queue_item = generate_queue_item();
    assert_ok!(store(queue_item, &app.db_pool).await);

    let get_item_result = get_item_that_needs_worked(10, &app.db_pool).await;
    assert_ok!(&get_item_result);

    assert_some!(get_item_result.unwrap());
}
