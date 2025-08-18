// midware/error.rs
use axum::{
    middleware::Next,
    response::{Html, IntoResponse, Response},
    extract::{Request, State},
    http::StatusCode,
};
use crate::handlers::{AppState, AppError};

pub async fn error_handler(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    
    // Check if the response is an error status
    if response.status().is_client_error() || response.status().is_server_error() {
        // Try to extract AppError from response (this is tricky with current setup)
        // For now, just handle based on status code
        let error_html = match response.status() {
            StatusCode::NOT_FOUND => render_error_template(&state, AppError::NotFound).await,
            _ => render_error_template(&state, AppError::Internal("Server error".to_string())).await,
        };
        
        return (response.status(), Html(error_html)).into_response();
    }
    
    response
}

async fn render_error_template(state: &AppState, error: AppError) -> String {
    let mut context = tera::Context::new();
    
    let (status_code, title, message) = match error {
        AppError::NotFound => (404, "Page Not Found", "The page you're looking for doesn't exist."),
        AppError::Internal(_) => (500, "Server Error", "Something went wrong on our end."),
    };
    
    context.insert("status_code", &status_code);
    context.insert("title", &title);
    context.insert("message", &message);
    
    // Try to render template, fall back to basic HTML if it fails
    state.tera
        .render("error.html", &context)
        .unwrap_or_else(|_| {
            format!(
                r#"<!DOCTYPE html>
<html><head><title>Error {}</title></head>
<body style="font-family: Arial; text-align: center; margin-top: 100px;">
    <h1>{}</h1><p>{}</p><a href="/">Go Home</a>
</body></html>"#,
                status_code, status_code, message
            )
        })
}