use crate::domain::file_scan_model::{FileScan, ScanStatus};
use crate::util::{get_unix_epoch_time_as_seconds, get_unix_epoch_time_minus_minutes_as_seconds};
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

#[tracing::instrument(name = "Get a file hash by id", skip(pool))]
pub async fn select_a_file_hash_by_id(id: Uuid, pool: &PgPool) -> Result<FileScan, Error> {
    let result = sqlx::query!(
        r#"SELECT
            id,
            file_name,
            file_location,
            file_hash,
            posted_on,
            last_updated,
            status,
            being_worked,
            work_started
          FROM file_scan
          WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e: Error| {
        tracing::error!("{:?}", e);
        e
    })?;

    Ok(FileScan {
        id: result.id,
        file_name: result.file_name,
        file_location: result.file_location,
        file_hash: result.file_hash,
        posted_on: result.posted_on,
        last_updated: result.last_updated,
        status: ScanStatus::from_str(result.status.as_str()).unwrap(),
        being_worked: result.being_worked,
        work_started: result.work_started,
    })
}

pub static MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_HASH_AGAIN: i64 = 15;

#[tracing::instrument(name = "Select a file that needs hashing", skip(pool))]
pub async fn select_a_file_that_needs_hashing(pool: &PgPool) -> Result<Option<FileScan>, Error> {
    let work_start_time = get_unix_epoch_time_as_seconds() as i64;
    let abandoned_time = get_unix_epoch_time_minus_minutes_as_seconds(
        MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_HASH_AGAIN,
    ) as i64;
    let result = sqlx::query!(
        r#"UPDATE file_scan
            SET
                being_worked = true,
                work_started = $1,
                status = $2
            WHERE id = (
                SELECT id
                FROM file_scan
                WHERE
                    (status = $3 AND being_worked = false) OR (status = $4 AND work_started <= $5)
                LIMIT 1
                FOR    UPDATE SKIP LOCKED
            )
            RETURNING *"#,
        Some(work_start_time),
        ScanStatus::Hashing.as_str(),
        ScanStatus::Pending.as_str(),
        ScanStatus::Hashing.as_str(),
        abandoned_time,
    )
    .fetch_optional(pool)
    .await;

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

#[tracing::instrument(name = "Set a file to be done with hashing", skip(id, pool))]
pub async fn set_a_file_scan_to_be_done_hashing(
    id: Uuid,
    hash: String,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query!(
        r#"UPDATE file_scan
            SET
                being_worked = false,
                status = $1,
                file_hash = $2
            WHERE id = $3"#,
        ScanStatus::DoneHashing.as_str(),
        hash,
        id,
    )
    .execute(pool)
    .await
    .map_err(|e: Error| {
        tracing::error!("{:?}", e);
        e
    })?;

    Ok(())
}
