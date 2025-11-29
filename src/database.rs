use crate::models::{Paste, PasteForm};
use anyhow::Result;
use chrono::Utc;
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;
use tracing::{debug, info};

const DB_URL: &str = "sqlite:pastes.db";

/// Sets up the database connection pool and runs migrations
pub async fn setup_database() -> Result<Pool<Sqlite>> {
    info!("Setting up database connection pool and running migrations.");
    let connect_options = SqliteConnectOptions::from_str(DB_URL)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Database setup complete.");
    Ok(pool)
}

/// Creates a new paste in the database
pub async fn create_paste(pool: &Pool<Sqlite>, form: &PasteForm) -> Result<String> {
    let id = nanoid::nanoid!(12); // todo: move key length to site settings
    debug!(paste_id = %id, "Inserting new paste into database.");

    let expires_in_seconds = match form.expiration.as_str() {
        "10m" => Some(10 * 60),
        "1h" => Some(60 * 60),
        "1d" => Some(24 * 60 * 60),
        "1w" => Some(7 * 24 * 60 * 60),
        _ => None,
    };

    let max_views = form.max_views.parse::<i64>().ok().map(|v| v + 1);

    sqlx::query(
        "INSERT INTO pastes (id, content, created_at, expires_in_seconds, max_views) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&form.content)
    .bind(Utc::now())
    .bind(expires_in_seconds)
    .bind(max_views)
    .execute(pool)
    .await?;
    Ok(id)
}

/// Retrieves a paste from the database by its ID
pub async fn get_paste(pool: &Pool<Sqlite>, id: &str) -> Result<Paste> {
    debug!(paste_id = %id, "Querying database for paste.");
    let paste = sqlx::query_as::<_, Paste>(
        "SELECT id, content, created_at, expires_in_seconds, max_views, views FROM pastes WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(paste)
}

/// Increments the view count for a paste
pub async fn increment_views(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
    debug!(paste_id = %id, "Incrementing view count.");
    sqlx::query("UPDATE pastes SET views = views + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Deletes a paste from the database by its ID
pub async fn delete_paste(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
    info!(paste_id = %id, "Deleting paste from database.");
    sqlx::query("DELETE FROM pastes WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
