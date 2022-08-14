use crate::configuration::get_configuration;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use futures_util::StreamExt;
use sqlx::PgPool;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::db::file_scan_broker::insert_scan;
use crate::domain::file_scan_model::{FileScan, ScanStatus};

#[tracing::instrument(name = "Post a file to scan", skip(payload, pool))]
pub async fn scan_file(mut payload: web::Payload, pool: web::Data<PgPool>) -> impl Responder {
    let filename = Uuid::new_v4().to_string();
    let scan_config = get_configuration().unwrap().scan_settings;
    let filepath = format!("{}/{}", scan_config.download_dir, filename);
    let mut file = File::create(filepath.clone()).await.unwrap();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        file.write_all(&chunk).await.unwrap();
    }

    let file_scan = FileScan {
        id: Uuid::new_v4(),
        file_name: filename.clone(),
        file_location: filepath.clone(),
        file_hash: String::new(),
        posted_on: Utc::now(),
        last_updated: Utc::now(),
        status: ScanStatus::Pending,
        being_worked: false,
        work_started: Some(0),
        scan_result: None,
        scan_result_details: None,
    };

    match insert_scan(file_scan.clone(), &pool).await {
        Ok(_) => HttpResponse::Ok().json(file_scan.clone()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
