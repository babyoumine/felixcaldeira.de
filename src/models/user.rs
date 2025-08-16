use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc, NaiveDateTime};
use sqlx::FromRow;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<u32>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// fn serialize_datetime<S>(value: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let utc_dt = DateTime::<Utc>::from_naive_utc_and_offset(*value, Utc);
//     utc_dt.serialize(serializer)
// }

// #[derive(Debug, Deserialize)]
// pub struct CreateUser {
//     pub username: String,
//     pub email: String,
//     pub password: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct LoginRequest {
//     pub username: String,
//     pub password: String,
// }

// impl User {
//     /// Find user by username
//     pub async fn find_by_username(pool: &sqlx::MySqlPool, username: &str) -> Result<Option<Self>, sqlx::Error> {
//         sqlx::query_as!(
//             User,
//             "SELECT * FROM users WHERE username = ?",
//             username
//         )
//         .fetch_optional(pool)
//         .await
//     }
    
//     /// Find user by email
//     pub async fn find_by_email(pool: &sqlx::MySqlPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
//         sqlx::query_as!(
//             User,
//             "SELECT * FROM users WHERE email = ?",
//             email
//         )
//         .fetch_optional(pool)
//         .await
//     }
    
//     /// Find user by ID
//     pub async fn find_by_id(pool: &sqlx::MySqlPool, id: u32) -> Result<Option<Self>, sqlx::Error> {
//         sqlx::query_as!(
//             User,
//             "SELECT * FROM users WHERE id = ?",
//             id
//         )
//         .fetch_optional(pool)
//         .await
//     }
    
//     /// Create a new user with hashed password
//     pub async fn create(pool: &sqlx::MySqlPool, user_data: CreateUser) -> Result<u32, Box<dyn std::error::Error>> {
//         // Hash the password
//         let password_hash = hash(&user_data.password, DEFAULT_COST)?;
        
//         // Insert user
//         let result = sqlx::query!(
//             "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
//             user_data.username,
//             user_data.email,
//             password_hash
//         )
//         .execute(pool)
//         .await?;
        
//         Ok(result.last_insert_id() as u32)
//     }
    
//     /// Verify password against stored hash
//     pub fn verify_password(&self, password: &str) -> bool {
//         verify(password, &self.password_hash).unwrap_or(false)
//     }
    
//     /// Authenticate user with username and password
//     pub async fn authenticate(
//         pool: &sqlx::MySqlPool, 
//         login: LoginRequest
//     ) -> Result<Option<Self>, Box<dyn std::error::Error>> {
//         if let Some(user) = Self::find_by_username(pool, &login.username).await? {
//             if user.verify_password(&login.password) {
//                 return Ok(Some(user));
//             }
//         }
//         Ok(None)
//     }
    
//     /// Update user password
//     pub async fn update_password(
//         &self,
//         pool: &sqlx::MySqlPool,
//         new_password: &str
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         let password_hash = hash(new_password, DEFAULT_COST)?;
        
//         sqlx::query!(
//             "UPDATE users SET password_hash = ? WHERE id = ?",
//             password_hash,
//             self.id
//         )
//         .execute(pool)
//         .await?;
        
//         Ok(())
//     }
    
//     /// Check if username is available
//     pub async fn username_available(pool: &sqlx::MySqlPool, username: &str) -> Result<bool, sqlx::Error> {
//         let count = sqlx::query!(
//             "SELECT COUNT(*) as count FROM users WHERE username = ?",
//             username
//         )
//         .fetch_one(pool)
//         .await?;
        
//         Ok(count.count == 0)
//     }
    
//     /// Check if email is available
//     pub async fn email_available(pool: &sqlx::MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
//         let count = sqlx::query!(
//             "SELECT COUNT(*) as count FROM users WHERE email = ?",
//             email
//         )
//         .fetch_one(pool)
//         .await?;
        
//         Ok(count.count == 0)
//     }
    
//     /// Get all users (for admin purposes)
//     pub async fn find_all(pool: &sqlx::MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
//         sqlx::query_as!(
//             User,
//             "SELECT * FROM users ORDER BY created_at DESC"
//         )
//         .fetch_all(pool)
//         .await
//     }
    
//     /// Count total users
//     pub async fn count(pool: &sqlx::MySqlPool) -> Result<i64, sqlx::Error> {
//         let result = sqlx::query!(
//             "SELECT COUNT(*) as count FROM users"
//         )
//         .fetch_one(pool)
//         .await?;
        
//         Ok(result.count)
//     }
// }

// /// JWT Claims structure for authentication
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub user_id: u32,
//     pub username: String,
//     pub exp: usize, // Expiration time
// }

// impl Claims {
//     pub fn new(user: &User, expiration_hours: i64) -> Self {
//         let exp = (chrono::Utc::now() + chrono::Duration::hours(expiration_hours)).timestamp() as usize;
        
//         Claims {
//             user_id: user.id,
//             username: user.username.clone(),
//             exp,
//         }
//     }
// }