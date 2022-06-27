use claim::{assert_ok, assert_some};
use file_scanner::db::file_scan_broker::{insert_scan, select_a_file_that_needs_hashing};
use file_scanner::domain::file_scan_model::ScanStatus;
use crate::helper::{generate_file_scan, spawn_app};


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
    assert_ok!(insert_scan(file_scan, &app.db_pool).await);
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

    assert_eq!(file_scan.id, returned_scan.unwrap().id);
}

#[tokio::test]
async fn select_a_file_that_needs_hashing_does_not_find_anything() {
    let app = spawn_app().await;

    let returned = select_a_file_that_needs_hashing(&app.db_pool).await;
    assert_ok!(&returned);

}