use axum::{
    extract::State,
    response::Html,
    http::StatusCode,
};
use tera::Context;
use crate::handlers::AppState;

pub async fn dashboard(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    context.insert("page_title", "Admin Dashboard");
    
    let html = state.tera
        .render("admin/dashboard.html", &context)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}

pub async fn login(State(_state): State<AppState>) -> Result<Html<String>, StatusCode> {
    // TODO: Implement login logic
    Ok(Html("Login endpoint".to_string()))
}

pub async fn logout(State(_state): State<AppState>) -> Result<Html<String>, StatusCode> {
    // TODO: Implement logout logic
    Ok(Html("Logout endpoint".to_string()))
}