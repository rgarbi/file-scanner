use crate::domain::file_scan_model::{FileScan, ScanResult, ScanStatus};
use crate::util::{get_unix_epoch_time_as_seconds, get_unix_epoch_time_minus_minutes_as_seconds};
use sqlx::{Error, PgPool};
use std::str::FromStr;
use uuid::Uuid;
use crate::background::queue_item::QueueItem;

#[tracing::instrument(name = "Saving new file scan", skip(queue_item, pool))]
pub async fn store(queue_item: QueueItem, pool: &PgPool) -> Result<Uuid, Error> {
    sqlx::query!(
        r#"INSERT
            INTO queue_items (id, queue_item_type, queue_item_contents, work_started, being_worked, error_count, error_message)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        queue_item.id,
        queue_item.queue_item_type,
        queue_item.queue_item_contents,
        queue_item.work_started,
        queue_item.being_worked,
        queue_item.error_count,
        queue_item.error_message.as_str(),
    ).execute(pool)
        .await
        .map_err(|e: Error| {
            tracing::error!("{:?}", e);
            e
        })?;

    Ok(queue_item.id)
}