use crate::helper::{send_file, spawn_app};
use claim::{assert_err, assert_ok};
use file_scanner::background::background_hasher::hash_files;
use file_scanner::db::file_scan_broker::select_a_file_hash_by_id;
use file_scanner::domain::file_scan_model::ScanStatus;

#[tokio::test]
async fn post_file_to_file_scan_works() {
    let app = spawn_app(false).await;

    let file_scan_stored = send_file(&app).await;

    assert_eq!(&file_scan_stored.file_hash, "");
    assert_eq!(&file_scan_stored.status, &ScanStatus::Pending);

    hash_files(&app.db_pool).await;

    let after_hash = select_a_file_hash_by_id(file_scan_stored.clone().id, &app.db_pool).await;
    assert_ok!(&after_hash);

    let file_scan_after_hash = after_hash.unwrap();

    assert_ne!(&file_scan_after_hash.file_hash, "");
    assert_eq!(&file_scan_after_hash.status, &ScanStatus::DoneHashing);
}

#[tokio::test]
async fn post_file_to_file_scan_2x_does_the_right_thing() {
    let app = spawn_app(false).await;

    let file_scan_stored = send_file(&app).await;
    assert_eq!(&file_scan_stored.file_hash, "");
    assert_eq!(&file_scan_stored.status, &ScanStatus::Pending);

    hash_files(&app.db_pool).await;

    let after_hash = select_a_file_hash_by_id(file_scan_stored.clone().id, &app.db_pool).await;
    assert_ok!(&after_hash);

    let file_scan_after_hash = after_hash.unwrap();

    assert_ne!(&file_scan_after_hash.file_hash, "");
    assert_eq!(&file_scan_after_hash.status, &ScanStatus::DoneHashing);

    hash_files(&app.db_pool).await;

    let after_hash_second_hash =
        select_a_file_hash_by_id(file_scan_stored.clone().id, &app.db_pool).await;
    assert_ok!(&after_hash_second_hash);

    let file_scan_after_second_hash = after_hash_second_hash.unwrap();

    assert_ne!(&file_scan_after_second_hash.file_hash, "");
    assert_eq!(
        &file_scan_after_second_hash.status,
        &ScanStatus::DoneHashing
    );
}

#[tokio::test]
async fn post_file_to_file_scan_blows_up_when_fetching_file() {
    let app = spawn_app(false).await;
    let file_scan_stored = send_file(&app).await;

    assert_eq!(&file_scan_stored.file_hash, "");
    assert_eq!(&file_scan_stored.status, &ScanStatus::Pending);

    // Sabotage the database
    sqlx::query!("ALTER TABLE file_scan DROP COLUMN file_name;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    hash_files(&app.db_pool).await;

    let after_hash = select_a_file_hash_by_id(file_scan_stored.clone().id, &app.db_pool).await;
    assert_err!(&after_hash);
}
