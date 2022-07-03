use sqlx::PgPool;

pub async fn scan_files(_pg_pool: &PgPool) {
    //get a lock on a file that has been hashed but has not been scanned.
    //scan the file.
    //update the record
}
