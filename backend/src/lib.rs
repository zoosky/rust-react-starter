use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

pub mod database;
pub mod models;
pub mod handlers;
#[cfg(test)]
pub mod test_utils;

use database::create_database_pool;
use handlers::{users, projects};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct HelloRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    greeting: String,
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("OK".to_string()),
        message: Some("Service is healthy".to_string()),
    })
}

async fn hello_handler(Json(payload): Json<HelloRequest>) -> Json<ApiResponse<HelloResponse>> {
    let greeting = format!("Hello, {}! Welcome to Rust + React starter.", payload.name);
    
    Json(ApiResponse {
        success: true,
        data: Some(HelloResponse { greeting }),
        message: Some("Greeting generated successfully".to_string()),
    })
}

pub fn create_app(pool: sqlx::SqlitePool) -> Router {
    Router::new()
        // Health and hello endpoints
        .route("/api/health", get(health_check))
        .route("/api/hello", post(hello_handler))
        // User endpoints
        .route("/api/users", get(users::get_users).post(users::create_user))
        .route("/api/users/:id", get(users::get_user).put(users::update_user).delete(users::delete_user))
        // Project endpoints
        .route("/api/projects", get(projects::get_projects).post(projects::create_project))
        .with_state(pool)
        .layer(CorsLayer::permissive())
}