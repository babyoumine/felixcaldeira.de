use axum::{
    extract::State,
    response::Html,
    http::StatusCode,
};
use tera::Context;
use crate::handlers::AppState;
use crate::models::Photo;

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    
    let photos = Photo::find_all_published(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    context.insert("photos", &photos);
    context.insert("page_title", "Photography");
    
    let html = state.tera
        .render("photos.html", &context)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}