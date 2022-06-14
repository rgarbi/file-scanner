use uuid::Uuid;
use file_scanner::domain::file_scan_model::Sample;

use crate::helper::spawn_app;

#[tokio::test]
async fn post_sample_works() {
    let app = spawn_app().await;

    let input: &[u8] = include_bytes!("../../tests/test_files/sample_file_1.txt");
    let body = Vec::from(input);

    let response = app.post_scan(body).await;

    // Assert
    assert!(&response.status().is_success());
    println!("Response: {:?}", response.unwrap())
}
