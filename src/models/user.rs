// src/models/user.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use sqlx::FromRow;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn find_by_username(pool: &sqlx::MySqlPool, username: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = ?",
            username
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_id(pool: &sqlx::MySqlPool, id: u32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &sqlx::MySqlPool, 
        username: &str, 
        email: &str, 
        password: &str
    ) -> Result<u32, sqlx::Error> {
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|_| sqlx::Error::Protocol("Failed to hash password".into()))?;

        let result = sqlx::query!(
            "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
            username,
            email,
            password_hash
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}