use axum::{
    response::{Html, IntoResponse, Response},
    extract::{State},
    http::StatusCode,
};
use serde::Serialize;

// Enhanced error type
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Template(tera::Error),
    NotFound,
    Unauthorized,
    BadRequest(String),
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Template(e) => write!(f, "Template error: {:#?}", e),
            AppError::NotFound => write!(f, "Page not found"),
            AppError::Unauthorized => write!(f, "Unauthorized access"),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

pub async fn handler_404(State(state): State<crate::handlers::AppState>) -> Html<String> {
    let mut context = tera::Context::new();
    context.insert("status_code", &404);
    context.insert("error_message", &"Page not found");
    
    let html = state.tera
        .render("error.html", &context)
        .unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());
    
    Html(html)
}