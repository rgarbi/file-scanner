use crate::domain::file_scan_model::{FileScan, ScanStatus};
use sqlx::{Error, PgPool};
use std::str::FromStr;
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

#[tracing::instrument(name = "Select a file that needs hashing", skip(pool))]
pub async fn select_a_file_that_needs_hashing(pool: &PgPool) -> Result<Option<FileScan>, Error> {
    let result = sqlx::query!(
        r#"SELECT id, file_name, file_location, file_hash, posted_on, last_updated, status, being_worked, work_started
            FROM file_scan
            WHERE status = $1"#,
        ScanStatus::Pending.as_str()
    ).fetch_optional(pool).await;

    return match result {
        Ok(res) => match res {
            Some(row) => Ok(Some(FileScan {
                id: row.id,
                file_name: row.file_name,
                file_location: row.file_location,
                file_hash: row.file_hash,
                posted_on: row.posted_on,
                last_updated: row.last_updated,
                status: ScanStatus::from_str(row.status.as_str()).unwrap(),
                being_worked: row.being_worked,
                work_started: row.work_started,
            })),
            None => Ok(None),
        },
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(e)
        }
    };
}
