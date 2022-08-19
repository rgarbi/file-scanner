use crate::helper::{generate_queue_item, spawn_app};
use claim::{assert_err, assert_none, assert_ok, assert_some};
use db::queue_item_broker::store;
use file_scanner::background::MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN;
use file_scanner::db;
use file_scanner::db::queue_item_broker::{get_item_that_needs_worked, put_item_back};
use file_scanner::util::get_unix_epoch_time_minus_minutes_as_seconds;

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

    let get_item_result = get_item_that_needs_worked(&app.db_pool).await;
    assert_ok!(&get_item_result);

    assert_some!(get_item_result.unwrap());
}

#[tokio::test]
async fn get_queue_item_blows_up() {
    let app = spawn_app(false).await;

    let queue_item = generate_queue_item();
    assert_ok!(store(queue_item, &app.db_pool).await);

    // Sabotage the database
    sqlx::query!("ALTER TABLE queue_items DROP COLUMN queue_item_contents;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    let get_item_result = get_item_that_needs_worked(&app.db_pool).await;
    assert_err!(&get_item_result);
}

#[tokio::test]
async fn get_queue_item_two_times_works() {
    let app = spawn_app(false).await;

    let queue_item = generate_queue_item();
    assert_ok!(store(queue_item, &app.db_pool).await);

    let get_item_result = get_item_that_needs_worked(&app.db_pool).await;
    assert_ok!(&get_item_result);
    assert_some!(get_item_result.unwrap());

    let get_item_result_2 = get_item_that_needs_worked(&app.db_pool).await;
    assert_ok!(&get_item_result_2);
    assert_none!(get_item_result_2.unwrap());
}

#[tokio::test]
async fn get_queue_item_that_is_expired_works() {
    let app = spawn_app(false).await;

    let mut queue_item = generate_queue_item();
    queue_item.being_worked = true;
    queue_item.work_started = Some(get_unix_epoch_time_minus_minutes_as_seconds(
        MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN + 10,
    ) as i64);
    assert_ok!(store(queue_item, &app.db_pool).await);

    let get_item_result = get_item_that_needs_worked(&app.db_pool).await;
    assert_ok!(&get_item_result);

    assert_some!(get_item_result.unwrap());
}

#[tokio::test]
async fn put_queue_item_back_works() {
    let app = spawn_app(false).await;

    let queue_item = generate_queue_item();
    assert_ok!(store(queue_item.clone(), &app.db_pool).await);

    let get_item_result = get_item_that_needs_worked(&app.db_pool).await;
    assert_ok!(&get_item_result);
    assert_some!(get_item_result.unwrap());

    assert_ok!(put_item_back(queue_item, &app.db_pool).await);

    let get_item_result_again = get_item_that_needs_worked(&app.db_pool).await;
    assert_ok!(&get_item_result_again);

    let item = get_item_result_again.unwrap();
    assert_some!(&item);

    assert_eq!(1, item.unwrap().error_count)
}

#[tokio::test]
async fn put_queue_item_back_errors() {
    let app = spawn_app(false).await;

    let queue_item = generate_queue_item();
    assert_ok!(store(queue_item.clone(), &app.db_pool).await);

    let get_item_result = get_item_that_needs_worked(&app.db_pool).await;
    assert_ok!(&get_item_result);
    assert_some!(get_item_result.unwrap());

    // Sabotage the database
    sqlx::query!("ALTER TABLE queue_items DROP COLUMN error_count;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    assert_err!(put_item_back(queue_item, &app.db_pool).await);
}
