use std::sync::OnceLock;
use dashmap::DashSet;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

static SENT_CACHE: OnceLock<DashSet<u32>> = OnceLock::new();

fn sent_cache() -> &'static DashSet<u32> {
    SENT_CACHE.get_or_init(|| DashSet::new())
}

// Initialize the database
pub async fn open() -> SqlitePool {
    std::fs::create_dir_all("data").unwrap();

    let options = SqliteConnectOptions::new()
        .filename("data/database.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options)
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

    let ids: Vec<u32> = sqlx::query_scalar("SELECT id FROM sent")
        .fetch_all(&pool)
        .await
        .unwrap();

    for id in ids {
        sent_cache().insert(id);
    }

    pool
}

// Get the total amount of sent levels stored in the permanent cache
pub async fn get_total_sent_levels(pool: &SqlitePool) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM sent")
        .fetch_one(pool)
        .await
        .unwrap()
}

// Add a level to the permanent sent levels cache
pub async fn add_sent_level(pool: &SqlitePool, id: u32) {
    sent_cache().insert(id);

    sqlx::query("INSERT OR IGNORE INTO sent (id) VALUES (?)")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

// Check if a level's present in the permanent sent levels cache
pub async fn is_level_sent(pool: &SqlitePool, id: u32) -> bool {
    if sent_cache().contains(&id) {
        return true;
    }
    
    sqlx::query("SELECT 1 FROM sent WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap()
        .is_some()
}