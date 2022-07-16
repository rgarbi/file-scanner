use crate::helper::{send_file, spawn_app, TestApp};
use claim::{assert_err, assert_ok};
use file_scanner::background::background_hasher::hash_files;
use file_scanner::background::background_scanner::scan_files;
use file_scanner::db::file_scan_broker::select_a_file_hash_by_id;
use file_scanner::domain::file_scan_model::ScanStatus;

#[tokio::test]
async fn scan_files_works() {
    let app = spawn_app(false).await;

    get_a_file_ready_for_scanning(&app).await;

    scan_files(&app.db_pool).await;
}


pub async fn get_a_file_ready_for_scanning(app: &TestApp) {
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