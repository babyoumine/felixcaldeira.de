// src/handlers/admin.rs
use axum::{
    extract::{Request, State},
    response::{Html, Redirect, Response, IntoResponse},
    http::{StatusCode, header},
    Form,
};
use tera::Context;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use crate::handlers::AppState;
use crate::models::{User, LoginRequest, Project, Photo};
use crate::midware::auth::{Claims, get_user_from_request};

// Show login form or dashboard based on auth status
pub async fn dashboard(
    State(state): State<AppState>,
    request: Request,
) -> Result<Response, StatusCode> {
    let mut context = Context::new();
    
    // Check if user is authenticated
    if let Some(user) = get_user_from_request(&request) {
        // User is logged in - show dashboard
        context.insert("page_title", "Dashboard");
        context.insert("user", &user);
        
        // Get recent projects and photos for dashboard overview
        let projects = Project::find_recent(&state.db, 5)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let photos = Photo::find_recent(&state.db, 5)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        context.insert("recent_projects", &projects);
        context.insert("recent_photos", &photos);
        
        let html = state.tera
            .render("admin/dashboard.html", &context)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Html(html).into_response())
    } else {
        // User not logged in - show login form
        context.insert("page_title", "Login");
        
        let html = state.tera
            .render("admin/login.html", &context)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Html(html).into_response())
    }
}

// Show login form
pub async fn show_login(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    context.insert("page_title", "Login");
    
    let html = state.tera
        .render("admin/login.html", &context)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}

// Handle login form submission
pub async fn login(
    State(state): State<AppState>,
    Form(login_data): Form<LoginRequest>,
) -> Result<Response, StatusCode> {
    // Find user by username
    let user = User::find_by_username(&state.db, &login_data.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match user {
        Some(user) if user.verify_password(&login_data.password) => {
            // Create JWT token
            let expiration = Utc::now() + Duration::hours(24); // Token expires in 24 hours
            
            let claims = Claims {
                sub: user.id.to_string(),
                username: user.username.clone(),
                exp: expiration.timestamp() as usize,
            };
            
            let encoding_key = EncodingKey::from_secret(state.config.jwt_secret.as_bytes());
            let token = encode(&Header::default(), &claims, &encoding_key)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            // Set cookie and redirect to dashboard
            let cookie = format!("auth_token={}; HttpOnly; Path=/; Max-Age=86400", token);
            
            let mut response = Redirect::to("/admin").into_response();
            response.headers_mut().insert(
                header::SET_COOKIE,
                cookie.parse().unwrap()
            );
            Ok(response)
        }
        _ => {
            // Invalid credentials - show login form with error
            let mut context = Context::new();
            context.insert("page_title", "Login");
            context.insert("error", "Invalid username or password");
            
            let html = state.tera
                .render("admin/login.html", &context)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok(Html(html).into_response())
        }
    }
}

// Handle logout
pub async fn logout() -> Response {
    // Clear the auth cookie and redirect to login
    let cookie = "auth_token=; HttpOnly; Path=/; Max-Age=0";
    
    let mut response = Redirect::to("/admin").into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookie.parse().unwrap()
    );
    response
}

// Projects management
pub async fn projects_list(
    State(state): State<AppState>,
    request: Request,
) -> Result<Html<String>, StatusCode> {
    let user = get_user_from_request(&request).ok_or(StatusCode::UNAUTHORIZED)?;
    
    let mut context = Context::new();
    context.insert("page_title", "Manage Projects");
    context.insert("user", &user);
    
    // Get all projects (including unpublished)
    let projects = sqlx::query_as!(
        Project,
        "SELECT * FROM projects ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    context.insert("projects", &projects);
    
    let html = state.tera
        .render("admin/projects.html", &context)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}

// Photos management
pub async fn photos_list(
    State(state): State<AppState>,
    request: Request,
) -> Result<Html<String>, StatusCode> {
    let user = get_user_from_request(&request).ok_or(StatusCode::UNAUTHORIZED)?;
    
    let mut context = Context::new();
    context.insert("page_title", "Manage Photos");
    context.insert("user", &user);
    
    // Get all photos (including unpublished)
    let photos = sqlx::query_as!(
        Photo,
        "SELECT * FROM photos ORDER BY taken_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    context.insert("photos", &photos);
    
    let html = state.tera
        .render("admin/photos.html", &context)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(html))
}