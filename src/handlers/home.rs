use axum::{
    extract::State,
    response::Html,
    http::StatusCode,
};
use tera::Context;
use crate::handlers::AppState;
use crate::models::{Project, Photo, User};

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    
    let user = User::find_by_id(&state.db, 1)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // if user.is_none() {
    //     let user_id = User::create(&state.db, "admin", "example@example.com", "password")
    //         .await
    //         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    //     println!("{:?}", user_id);
    // } else {
    //     println!("User already created");
    // }

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