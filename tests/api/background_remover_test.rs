use file_scanner::background::background_remover::remove_files;
use crate::helper::{send_file, spawn_app};
use file_scanner::domain::file_scan_model::ScanStatus;

#[tokio::test]
async fn remove_files_works() {
    let app = spawn_app(false).await;

    let file_scan_stored = send_file(&app).await;

    assert_eq!(&file_scan_stored.file_hash, "");
    assert_eq!(&file_scan_stored.status, &ScanStatus::Pending);

    remove_files(&app.db_pool).await;
}
