use chrono::Utc;
use once_cell::sync::Lazy;
use reqwest::Response;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;
use file_scanner::background::queue_item::QueueItem;

use file_scanner::configuration::{get_configuration, DatabaseSettings};
use file_scanner::domain::file_scan_model::{FileScan, ScanStatus};
use file_scanner::startup::Application;
use file_scanner::telemetry::{get_subscriber, init_subscriber};

pub static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn post_scan(&self, body: Vec<u8>) -> Response {
        reqwest::Client::new()
            .post(&format!("{}/file_scanner", &self.address))
            .header("Content-Type", "application/octet-stream")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app(enable_background_processing: bool) -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c.email_client.base_url = email_server.uri();
        c.application.enable_background_processing = enable_background_processing;
        c
    };

    let pool = configure_database(&configuration.database).await;
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: pool,
        email_server,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

pub fn generate_file_scan() -> FileScan {
    FileScan {
        id: Uuid::new_v4(),
        file_name: Uuid::new_v4().to_string(),
        file_location: Uuid::new_v4().to_string(),
        file_hash: Uuid::new_v4().to_string(),
        posted_on: Utc::now(),
        last_updated: Utc::now(),
        status: ScanStatus::Pending,
        being_worked: false,
        work_started: None,
        scan_result: None,
        scan_result_details: None,
    }
}

pub fn generate_queue_item() -> QueueItem {
    QueueItem {
        id: Default::default(),
        queue_item_type: "item type".to_string(),
        queue_item_contents: generate_file_scan().to_json(),
        work_started: Some(0),
        being_worked: false,
        error_count: 0,
        error_message: None
    }
}

pub async fn send_file(app: &TestApp) -> FileScan {
    let input: &[u8] = include_bytes!("../../tests/test_files/sample_file_1.txt");
    let body = Vec::from(input);
    let response = app.post_scan(body).await;

    assert!(&response.status().is_success());
    to_file_scan_from_str(response.text().await.unwrap().as_str())
}

pub fn to_file_scan_from_str(file_scan: &str) -> FileScan {
    serde_json::from_str(file_scan).unwrap()
}
