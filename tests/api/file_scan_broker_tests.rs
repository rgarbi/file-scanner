use crate::helper::{generate_file_scan, spawn_app};
use claim::{assert_err, assert_ge, assert_none, assert_ok, assert_some};
use file_scanner::db::file_scan_broker::{
    insert_scan, select_a_file_that_needs_hashing, select_all_file_hashes_by_status,
    MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_HASH_AGAIN,
};
use file_scanner::domain::file_scan_model::ScanStatus;
use file_scanner::util::{
    get_unix_epoch_time_as_seconds, get_unix_epoch_time_minus_minutes_as_seconds,
};
use uuid::Uuid;

#[tokio::test]
async fn insert_scan_works() {
    let app = spawn_app(false).await;

    let file_scan = generate_file_scan();
    assert_ok!(insert_scan(file_scan, &app.db_pool).await);
}

#[tokio::test]
async fn insert_scan_errors() {
    let app = spawn_app(false).await;

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
    let app = spawn_app(false).await;

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
async fn select_a_file_among_many_that_needs_hashing_works() {
    let app = spawn_app(false).await;

    let mut file_scan_ids: Vec<Uuid> = Vec::new();

    for _ in 0..10 {
        let mut file_scan = generate_file_scan();
        file_scan.status = ScanStatus::Pending;
        let result = insert_scan(file_scan.clone(), &app.db_pool).await;
        assert_ok!(&result);

        file_scan_ids.push(result.unwrap());
    }

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);
    let returned_scan = returned.unwrap();
    assert_some!(&returned_scan);

    let get_by_status_result =
        select_all_file_hashes_by_status(ScanStatus::Pending, &app.db_pool).await;
    assert_ok!(&get_by_status_result);

    let scans = get_by_status_result.unwrap();
    assert_eq!(9, scans.len());
}

#[tokio::test]
async fn select_a_file_that_needs_hashing_does_not_find_anything() {
    let app = spawn_app(false).await;

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);

    let returned_scan = returned.unwrap();
    assert_none!(&returned_scan);
}

#[tokio::test]
async fn select_a_file_that_needs_hashing_because_it_was_abandoned_works() {
    let app = spawn_app(false).await;

    let mut file_scan = generate_file_scan();
    file_scan.status = ScanStatus::Hashing;
    file_scan.work_started = Some(
        get_unix_epoch_time_minus_minutes_as_seconds(
            MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_HASH_AGAIN,
        ) as i64
            - 5,
    );
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
    let app = spawn_app(false).await;

    let mut file_scan = generate_file_scan();
    file_scan.status = ScanStatus::Pending;
    file_scan.work_started = Some(get_unix_epoch_time_minus_minutes_as_seconds(
        MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_HASH_AGAIN - 1,
    ) as i64);
    file_scan.being_worked = true;

    assert_ok!(insert_scan(file_scan.clone(), &app.db_pool).await);

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);

    let returned_scan = returned.unwrap();
    assert_none!(&returned_scan);
}

#[tokio::test]
async fn select_a_file_that_is_stuck_hashing() {
    let app = spawn_app(false).await;

    let mut file_scan = generate_file_scan();
    file_scan.status = ScanStatus::Hashing;
    file_scan.work_started = Some(get_unix_epoch_time_minus_minutes_as_seconds(
        MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_HASH_AGAIN + 1,
    ) as i64);
    file_scan.being_worked = true;

    assert_ok!(insert_scan(file_scan.clone(), &app.db_pool).await);

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);

    let returned_scan = returned.unwrap();
    assert_some!(&returned_scan);
}

#[tokio::test]
async fn select_all_file_hashes_by_status_works() {
    let app = spawn_app(false).await;

    let file_scan = generate_file_scan();
    assert_ok!(insert_scan(file_scan, &app.db_pool).await);

    let get_by_status_result =
        select_all_file_hashes_by_status(ScanStatus::Pending, &app.db_pool).await;
    assert_ok!(&get_by_status_result);

    let scans = get_by_status_result.unwrap();
    assert_eq!(1, scans.len());
}

#[tokio::test]
async fn select_all_file_hashes_by_status_size_zero_works() {
    let app = spawn_app(false).await;

    let get_by_status_result =
        select_all_file_hashes_by_status(ScanStatus::Pending, &app.db_pool).await;
    assert_ok!(&get_by_status_result);

    let scans = get_by_status_result.unwrap();
    assert_eq!(0, scans.len());
}

#[tokio::test]
async fn select_all_file_hashes_by_status_errors() {
    let app = spawn_app(false).await;

    let file_scan = generate_file_scan();
    assert_ok!(insert_scan(file_scan, &app.db_pool).await);

    // Sabotage the database
    sqlx::query!("ALTER TABLE file_scan DROP COLUMN file_name;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    let get_by_status_result =
        select_all_file_hashes_by_status(ScanStatus::Pending, &app.db_pool).await;
    assert_err!(&get_by_status_result);
}
