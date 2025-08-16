use sqlx::MySqlPool;
use tera::Tera;
use crate::config::Config;

pub mod home;
pub mod about;
pub mod projects;
pub mod photography;
pub mod contact;
pub mod admin;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub tera: Tera,
    pub config: Config,
}