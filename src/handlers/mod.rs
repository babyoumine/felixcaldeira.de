use sqlx::MySqlPool;
use tera::Tera;
use crate::config::Config;

pub mod home;
pub mod about;
pub mod projects;
pub mod photography;
pub mod contact;
pub mod admin;
pub mod error;

pub use error::{AppError, handler_404};

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub tera: Tera,
    pub config: Config,
}