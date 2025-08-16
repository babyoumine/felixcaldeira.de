use axum::{
    routing::{get, post},
    Router,
    extract::DefaultBodyLimit,
};
use tower_http::{services::ServeDir, cors::CorsLayer};
use std::net::SocketAddr;

mod config;
mod database;
mod models;
mod handlers;
mod middleware;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::from_env()?;
    
    // Setup database
    let db = database::setup(&config.database_url).await?;
    
    // Run migrations
    // database::migrate(&db).await?;
    
    // Setup Tera templates
    let tera = tera::Tera::new(&format!("{}/**/*.html", config.template_dir))?;
    
    // Create shared state
    let app_state = handlers::AppState {
        db,
        tera,
        config: config.clone(),
    };
    
    // Build router
    let app = Router::new()
        // Public routes
        .route("/", get(handlers::home::index))
        .route("/about", get(handlers::about::index))
        .route("/projects", get(handlers::projects::list))
        .route("/projects/:id", get(handlers::projects::detail))
        .route("/photography", get(handlers::photography::index))
        .route("/contact", get(handlers::contact::index))
        
        // Admin routes
        .route("/admin", get(handlers::admin::dashboard))
        .route("/login", post(handlers::admin::login))
        .route("/logout", post(handlers::admin::logout))
        
        // Static files
        .nest_service("/static", ServeDir::new(&config.static_dir))
        .nest_service("/uploads", ServeDir::new(&config.upload_dir))
        
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB max
        .with_state(app_state);
    
    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}