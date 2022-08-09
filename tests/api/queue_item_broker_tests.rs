use crate::helper::{generate_queue_item, spawn_app};
use db::queue_item_broker::store;
use claim::{assert_ok};
use file_scanner::db;


#[tokio::test]
async fn insert_scan_works() {
    let app = spawn_app(false).await;

    let queue_item = generate_queue_item();
    assert_ok!(store(queue_item, &app.db_pool).await);
}
