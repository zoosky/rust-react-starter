use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::SqlitePool;

use crate::models::{CreateProjectRequest, Project, UpdateProjectRequest};

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
}

pub async fn create_project(
    State(pool): State<SqlitePool>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<ApiResponse<Project>>, StatusCode> {
    let project = Project::new(request.name, request.description, request.user_id);

    let result = sqlx::query(
        "INSERT INTO projects (id, name, description, user_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&project.id)
    .bind(&project.name)
    .bind(&project.description)
    .bind(&project.user_id)
    .bind(&project.created_at)
    .bind(&project.updated_at)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(ApiResponse::success(project))),
        Err(e) => {
            tracing::error!("Failed to create project: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_projects(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<Vec<Project>>>, StatusCode> {
    let result = sqlx::query_as::<_, Project>(
        "SELECT id, name, description, user_id, created_at, updated_at FROM projects ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(projects) => Ok(Json(ApiResponse::success(projects))),
        Err(e) => {
            tracing::error!("Failed to fetch projects: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}