use crate::helper::{send_file, spawn_app};
use claim::{assert_err, assert_ok};
use file_scanner::background::background_hasher::hash_files;
use file_scanner::db::file_scan_broker::select_a_file_hash_by_id;
use file_scanner::domain::file_scan_model::ScanStatus;

#[tokio::test]
async fn remove_files_works() {
    let app = spawn_app(false).await;

    let file_scan_stored = send_file(&app).await;

    assert_eq!(&file_scan_stored.file_hash, "");
    assert_eq!(&file_scan_stored.status, &ScanStatus::Pending);

    remove_files(&app.db_pool).await;
}
