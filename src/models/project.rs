use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub image_url: Option<String>,
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProject {
    pub title: String,
    pub description: String,
    pub content: String,
    pub image_url: Option<String>,
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
    pub published: Option<i8>,
}

impl Project {
    pub async fn find_all_published(pool: &sqlx::MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE published = true ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
    }
    
    pub async fn find_by_id(pool: &sqlx::MySqlPool, id: u32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE id = ? AND published = true",
            id
        )
        .fetch_optional(pool)
        .await
    }
    
    pub async fn find_recent(pool: &sqlx::MySqlPool, limit: u32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE published = true ORDER BY created_at DESC LIMIT ?",
            limit
        )
        .fetch_all(pool)
        .await
    }
}