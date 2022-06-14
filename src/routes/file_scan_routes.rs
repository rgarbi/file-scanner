use std::fs::File;
use std::io::{BufReader, Read, Write};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use data_encoding::HEXUPPER;
use futures_util::StreamExt;
use ring::digest::{Context, Digest, SHA256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::sample_broker::insert_user;
use crate::domain::file_scan_model::{FileScan, Sample, ScanStatus};


#[tracing::instrument(
name = "Post a file to scan",
skip(body, _pool),
)]
pub async fn scan_file(mut body: web::Payload, _pool: web::Data<PgPool>) -> impl Responder {
    let filename = Uuid::new_v4().to_string();
    let filepath = format!("./tmp/{}", filename);
    while let Some(item) = body.next().await {
        let mut f = web::block(|| std::fs::File::create(filepath.clone())).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    let file_hash = hash_a_file(filepath.clone());

    let file_scan = FileScan {
        id: Uuid::new_v4(),
        file_name: filename.clone(),
        file_location: filepath.clone(),
        file_hash,
        posted_on: Utc::now(),
        last_updated: Utc::now(),
        status: ScanStatus::Pending
    };

    Ok(HttpResponse::Ok().json(file_scan))
}

fn hash_a_file(path: String) -> String {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    HEXUPPER.encode(digest.as_ref())
}

//pulled from: https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, E> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}
