use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

// Initialize the database
pub async fn open() -> SqlitePool {
    std::fs::create_dir_all("data").unwrap();

    let options: sqlx::sqlite::SqliteConnectOptions = SqliteConnectOptions::new()
        .filename("data/database.db")
        .create_if_missing(true);

    let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect_with(options)
        .await
        .unwrap();

    // This should make it much faster overall
    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sent (
            id INTEGER NOT NULL UNIQUE,
            PRIMARY KEY (id)
        );"
    )
    .execute(&pool)
    .await
    .unwrap();

    return pool;
}

pub async fn get_total_sent_levels(pool: &SqlitePool) -> i64 {
    return sqlx::query_scalar("SELECT COUNT(*) FROM sent")
        .fetch_one(pool)
        .await
        .unwrap()
}

// Add a level to the permanent sent levels cache
pub async fn add_sent_level(pool: &SqlitePool, id: u32) {
    sqlx::query("INSERT OR IGNORE INTO sent (id) VALUES (?)")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

// Check if a level's present in the permanent sent levels cache
pub async fn is_level_sent(pool: &SqlitePool, id: u32) -> bool {
    return sqlx::query("SELECT 1 FROM sent WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap()
        .is_some();
}