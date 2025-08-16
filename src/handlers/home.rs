use axum::{
    extract::State,
    response::Html,
    http::StatusCode,
};
use tera::Context;
use crate::handlers::AppState;
use crate::models::{Project, Photo};

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    
    // Get recent projects and photos
    let recent_projects = Project::find_recent(&state.db, 3)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let recent_photos = Photo::find_recent(&state.db, 6)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    context.insert("projects", &recent_projects);
    context.insert("photos", &recent_photos);
    context.insert("page_title", "Start");
    
    let html = state.tera
        .render("home.html", &context)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}