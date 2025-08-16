use axum::{
    extract::{State, Path},
    response::Html,
    http::StatusCode,
};
use tera::Context;
use crate::handlers::AppState;
use crate::models::Project;

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    
    let projects = Project::find_all_published(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    context.insert("projects", &projects);
    context.insert("page_title", "Projects");
    
    let html = state.tera
        .render("projects.html", &context)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<u32>
) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    
    let project = Project::find_by_id(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    context.insert("project", &project);
    context.insert("page_title", &project.title);
    
    let html = state.tera
        .render("project_detail.html", &context)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}