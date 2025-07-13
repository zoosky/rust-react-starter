use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::SqlitePool;

use crate::models::{CreateUserRequest, UpdateUserRequest, User};

#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let user = User::new(request.name, request.email);

    let result = sqlx::query(
        "INSERT INTO users (id, name, email, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user.id)
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.created_at)
    .bind(&user.updated_at)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(ApiResponse::success(user))),
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_users(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<Vec<User>>>, StatusCode> {
    let result = sqlx::query_as::<_, User>(
        "SELECT id, name, email, created_at, updated_at FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(users) => Ok(Json(ApiResponse::success(users))),
        Err(e) => {
            tracing::error!("Failed to fetch users: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let result = sqlx::query_as::<_, User>(
        "SELECT id, name, email, created_at, updated_at FROM users WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(user)) => Ok(Json(ApiResponse::success(user))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    // First check if user exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, created_at, updated_at FROM users WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await;

    let mut user = match existing_user {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Update fields if provided
    if let Some(name) = request.name {
        user.name = name;
    }
    if let Some(email) = request.email {
        user.email = email;
    }
    user.updated_at = chrono::Utc::now();

    let result = sqlx::query(
        "UPDATE users SET name = ?, email = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.updated_at)
    .bind(&user.id)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(ApiResponse::success(user))),
        Err(e) => {
            tracing::error!("Failed to update user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(Json(ApiResponse::success(())))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}