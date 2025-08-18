use axum::{
    extract::State,
    response::Html,
    http::StatusCode,
};
use tera::Context;
use crate::handlers::AppState;

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    context.insert("page_title", "Kontakt");
    
    let html = state.tera
        .render("contact.html", &context)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}