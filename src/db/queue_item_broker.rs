use crate::background::queue_item::QueueItem;
use crate::background::MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN;
use crate::util::{get_unix_epoch_time_as_seconds, get_unix_epoch_time_minus_minutes_as_seconds};
use sqlx::{Error, PgPool};
use uuid::Uuid;

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
        queue_item.error_message,
    ).execute(pool)
        .await
        .map_err(|e: Error| {
            tracing::error!("{:?}", e);
            e
        })?;

    Ok(queue_item.id)
}

#[tracing::instrument(name = "Select and lock a file by status", skip(pool))]
pub async fn get_item_that_needs_worked(pool: &PgPool) -> Result<Option<QueueItem>, Error> {
    let work_start_time = get_unix_epoch_time_as_seconds() as i64;
    let abandoned_time = get_unix_epoch_time_minus_minutes_as_seconds(
        MINUTES_TO_WAIT_BEFORE_ATTEMPTING_TO_WORK_AGAIN,
    ) as i64;
    let result = sqlx::query!(
        r#"UPDATE queue_items
            SET
                being_worked = true,
                work_started = $1
            WHERE id = (
                SELECT id
                FROM queue_items
                WHERE (being_worked = false) OR (work_started <= $2)
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING *"#,
        Some(work_start_time),
        abandoned_time,
    )
    .fetch_optional(pool)
    .await;

    match result {
        Ok(res) => match res {
            Some(row) => Ok(Some(QueueItem {
                id: row.id,
                queue_item_type: row.queue_item_type,
                being_worked: row.being_worked,
                error_count: row.error_count,
                work_started: Some(row.work_started),
                queue_item_contents: row.queue_item_contents,
                error_message: row.error_message,
            })),
            None => Ok(None),
        },
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(e)
        }
    }
}

#[tracing::instrument(
    name = "Put item back and indicate the worker failed to process",
    skip(pool)
)]
pub async fn put_item_back(queue_item: QueueItem, pool: &PgPool) -> Result<Uuid, Error> {
    sqlx::query!(
        r#"UPDATE queue_items
            SET
                being_worked = false,
                error_count = error_count + 1
            WHERE id = $1"#,
        queue_item.id,
    )
    .execute(pool)
    .await
    .map_err(|e: Error| {
        tracing::error!("{:?}", e);
        e
    })?;

    Ok(queue_item.id)
}
