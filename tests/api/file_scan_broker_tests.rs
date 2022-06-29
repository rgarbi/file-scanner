use crate::helper::{generate_file_scan, spawn_app};
use claim::{assert_err, assert_ge, assert_none, assert_ok, assert_some};
use file_scanner::db::file_scan_broker::{insert_scan, select_a_file_that_needs_hashing};
use file_scanner::domain::file_scan_model::ScanStatus;
use file_scanner::util::{
    get_unix_epoch_time_as_seconds, get_unix_epoch_time_minus_minutes_as_seconds,
};

#[tokio::test]
async fn insert_scan_works() {
    let app = spawn_app().await;

    let file_scan = generate_file_scan();
    assert_ok!(insert_scan(file_scan, &app.db_pool).await);
}

#[tokio::test]
async fn insert_scan_errors() {
    let app = spawn_app().await;

    // Sabotage the database
    sqlx::query!("ALTER TABLE file_scan DROP COLUMN file_name;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    let file_scan = generate_file_scan();
    assert_err!(insert_scan(file_scan, &app.db_pool).await);
}

#[tokio::test]
async fn select_a_file_that_needs_hashing_works() {
    let app = spawn_app().await;

    let mut file_scan = generate_file_scan();
    file_scan.status = ScanStatus::Pending;
    assert_ok!(insert_scan(file_scan.clone(), &app.db_pool).await);

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);

    let returned_scan = returned.unwrap();
    assert_some!(&returned_scan);

    let now = get_unix_epoch_time_as_seconds() as i64;

    assert_eq!(file_scan.id, returned_scan.clone().unwrap().id);
    assert_eq!(true, returned_scan.clone().unwrap().being_worked);
    assert_ge!(now, returned_scan.clone().unwrap().work_started.unwrap())
}

#[tokio::test]
async fn select_a_file_that_needs_hashing_does_not_find_anything() {
    let app = spawn_app().await;

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);

    let returned_scan = returned.unwrap();
    assert_none!(&returned_scan);
}

#[tokio::test]
async fn select_a_file_that_needs_hashing_because_it_was_abandoned_works() {
    let app = spawn_app().await;

    let mut file_scan = generate_file_scan();
    file_scan.status = ScanStatus::Pending;
    file_scan.work_started = Some(get_unix_epoch_time_minus_minutes_as_seconds(15) as i64 + 1);
    file_scan.being_worked = true;

    assert_ok!(insert_scan(file_scan.clone(), &app.db_pool).await);

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);

    let returned_scan = returned.unwrap();
    assert_some!(&returned_scan);

    let now = get_unix_epoch_time_as_seconds() as i64;

    assert_eq!(file_scan.id, returned_scan.clone().unwrap().id);
    assert_eq!(true, returned_scan.clone().unwrap().being_worked);
    assert_ge!(now, returned_scan.clone().unwrap().work_started.unwrap())
}

#[tokio::test]
async fn select_a_file_that_needs_hashing_does_not_get_a_file_still_being_worked() {
    let app = spawn_app().await;

    let mut file_scan = generate_file_scan();
    file_scan.status = ScanStatus::Pending;
    file_scan.work_started = Some(get_unix_epoch_time_minus_minutes_as_seconds(10) as i64);
    file_scan.being_worked = true;

    assert_ok!(insert_scan(file_scan.clone(), &app.db_pool).await);

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);

    let returned_scan = returned.unwrap();
    assert_none!(&returned_scan);
}

#[tokio::test]
async fn select_a_file_that_needs_hashing_does_not_get_a_file_still_being_worked() {
    let app = spawn_app().await;

    let mut file_scan = generate_file_scan();
    file_scan.status = ScanStatus::Pending;
    file_scan.work_started = Some(get_unix_epoch_time_minus_minutes_as_seconds(10) as i64);
    file_scan.being_worked = true;

    assert_ok!(insert_scan(file_scan.clone(), &app.db_pool).await);

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);

    let returned_scan = returned.unwrap();
    assert_none!(&returned_scan);
}
