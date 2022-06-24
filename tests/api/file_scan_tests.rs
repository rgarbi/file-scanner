use crate::helper::spawn_app;

#[tokio::test]
async fn post_file_to_file_scan_works() {
    let app = spawn_app().await;

    let input: &[u8] = include_bytes!("../../tests/test_files/sample_file_1.txt");
    let body = Vec::from(input);

    let response = app.post_scan(body).await;

    // Assert
    assert!(&response.status().is_success());
    println!("Response: {:?}", response.text().await.unwrap())
}

#[tokio::test]
async fn post_file_to_file_scan_throws_a_500_when_it_cannot_insert_the_record() {
    let app = spawn_app().await;

    let input: &[u8] = include_bytes!("../../tests/test_files/sample_file_1.txt");
    let body = Vec::from(input);

    // Sabotage the database
    sqlx::query!("ALTER TABLE file_scan DROP COLUMN file_name;",)
        .execute(&app.db_pool)
        .await
        .unwrap();
    let response = app.post_scan(body).await;

    // Assert
    assert!(&response.status().is_server_error());
    println!("Response: {:?}", response.text().await.unwrap())
}
