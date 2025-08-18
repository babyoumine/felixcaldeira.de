use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Photo {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub taken_at: Option<NaiveDateTime>,
    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Photo {
    pub async fn find_all_published(pool: &sqlx::MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Photo,
            "SELECT * FROM photos WHERE published = true ORDER BY taken_at DESC"
        )
        .fetch_all(pool)
        .await
    }
    
    pub async fn find_recent(pool: &sqlx::MySqlPool, limit: u32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Photo,
            "SELECT * FROM photos WHERE published = true ORDER BY taken_at DESC LIMIT ?",
            limit
        )
        .fetch_all(pool)
        .await
    }
}