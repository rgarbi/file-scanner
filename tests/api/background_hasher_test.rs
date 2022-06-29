use crate::helper::{spawn_app, to_file_scan_from_str};
use claim::assert_ok;
use file_scanner::background::background_hasher::hash_files;
use file_scanner::db::file_scan_broker::select_a_file_hash_by_id;
use file_scanner::domain::file_scan_model::ScanStatus;

#[tokio::test]
async fn post_file_to_file_scan_works() {
    let app = spawn_app().await;

    let input: &[u8] = include_bytes!("../../tests/test_files/sample_file_1.txt");
    let body = Vec::from(input);
    let response = app.post_scan(body).await;

    // Assert
    assert!(&response.status().is_success());
    let file_scan_stored = to_file_scan_from_str(response.text().await.unwrap().as_str());

    assert_eq!(&file_scan_stored.file_hash, "");
    assert_eq!(&file_scan_stored.status, &ScanStatus::Pending);

    hash_files(&app.db_pool).await;

    let after_hash = select_a_file_hash_by_id(file_scan_stored.clone().id, &app.db_pool).await;
    assert_ok!(&after_hash);

    let file_scan_after_hash = after_hash.unwrap();

    assert_ne!(&file_scan_after_hash.file_hash, "");
    assert_eq!(&file_scan_after_hash.status, &ScanStatus::DoneHashing);
}
