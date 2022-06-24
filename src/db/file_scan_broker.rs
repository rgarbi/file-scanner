use crate::domain::file_scan_model::FileScan;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[tracing::instrument(name = "Saving new file scan", skip(file_scan, pool))]
pub async fn insert_scan(file_scan: FileScan, pool: &PgPool) -> Result<Uuid, Error> {
    sqlx::query!(
        r#"INSERT
            INTO file_scan (id, file_name, file_location, file_hash, posted_on, last_updated, status, being_worked, work_started)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        file_scan.id,
        file_scan.file_name,
        file_scan.file_location,
        file_scan.file_hash,
        file_scan.posted_on,
        file_scan.last_updated,
        file_scan.status.as_str(),
        file_scan.being_worked,
        file_scan.work_started,
    ).execute(pool)
        .await
        .map_err(|e: Error| {
            tracing::error!("{:?}", e);
            e
        })?;

    Ok(file_scan.id)
}
