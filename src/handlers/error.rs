use axum::{
    middleware::Next,
    response::{Html, IntoResponse, Response},
    extract::{Request, State},
    http::StatusCode,
};
use crate::handlers::{AppState};

// Simple error type
#[derive(Debug)]
pub enum AppError {
    NotFound,
    Internal(String),
}

pub async fn handler_404(State(state): State<AppState>) -> Html<String> {
    let mut context = tera::Context::new();
    context.insert("status_code", &404);
    context.insert("message", &"Page not found");
    
    let html = state.tera
        .render("error.html", &context)
        .unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());
    
    Html(html)
}