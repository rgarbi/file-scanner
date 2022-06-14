use sqlx::{Error, PgPool};
use crate::domain::file_scan_model::{FileScan, Sample};


#[tracing::instrument(
    name = "Saving new sample",
    skip(file_scan, _pool)
)]
pub async fn insert_scan(
    file_scan: FileScan,
    _pool: &PgPool,
) -> Result<String, Error> {
    // Commenting out so that we dop not have to create a fake migration
    //sqlx::query!(
    //    r#"INSERT
    //        INTO sample (id, string, number, small_number)
    //        VALUES ($1, $2, $3, $4)"#,
    //    sample.id,
    //    sample.string,
    //    sample.number,
    //    sample.small_number,
    //)
    //    .execute(pool)
    //    .await
    //    .map_err(|e: Error| {
    //        tracing::error!("{:?}", e);
    //        e
    //    })?;

    Ok(sample.id.to_string())
}




