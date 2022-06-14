use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Buf;
use chrono::Utc;
use data_encoding::HEXUPPER;
use futures_util::StreamExt;
use ring::digest::{Context, Digest, SHA256};
use sqlx::PgPool;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::fs::File;
use uuid::Uuid;

use crate::db::sample_broker::insert_scan;
use crate::domain::file_scan_model::{FileScan, ScanStatus};


#[tracing::instrument(
name = "Post a file to scan",
skip(body, pool),
)]
pub async fn scan_file(mut body: web::Payload, pool: web::Data<PgPool>) -> impl Responder {
    let filename = Uuid::new_v4().to_string();
    let filepath = format!("./tmp/{}", filename);
    let mut file = File::create(filepath.clone()).await.unwrap();

    while let Some(item) = body.next().await {
        let chunk = item.unwrap();
        file.write(&*chunk.chunk()).await.unwrap();
    }

    let file_hash = hash_a_file(filepath.clone()).await;
    let file_scan = FileScan {
        id: Uuid::new_v4(),
        file_name: filename.clone(),
        file_location: filepath.clone(),
        file_hash,
        posted_on: Utc::now(),
        last_updated: Utc::now(),
        status: ScanStatus::Pending,
    };

    match insert_scan(file_scan.clone(), &pool).await {
        Ok(_) => return HttpResponse::Ok().json(file_scan.clone()),
        Err(_) => return HttpResponse::InternalServerError().finish()
    }
}

async fn hash_a_file(path: String) -> String {
    let input = File::open(path).await.unwrap();

    let digest = sha256_digest(input).await;

    HEXUPPER.encode(digest.as_ref())
}

async fn sha256_digest(mut file: File) -> Digest {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = file.read(&mut buffer).await.unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    context.finish()
}