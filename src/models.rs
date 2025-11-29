use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct Paste {
    pub id: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub expires_in_seconds: Option<i64>,
    pub max_views: Option<i64>,
    pub views: i64,
}

#[derive(Deserialize, Debug)]
pub struct PasteForm {
    pub content: String,
    #[serde(default)]
    pub expiration: String,
    #[serde(default)]
    pub max_views: String,
}
