use std::io::Write;
use actix_web::{web, HttpResponse, Responder};
use futures_util::StreamExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::sample_broker::insert_user;
use crate::domain::file_scan_model::Sample;


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

    

    Ok(HttpResponse::Ok().finish())
}
