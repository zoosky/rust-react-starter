use axum::http::StatusCode;
use axum_test::TestServer;
use serde_json::{json, Value};
use serial_test::serial;

// Import from the binary crate
use rust_react_starter::create_app;

// We need to create our own test helpers since we can't import them from the binary
mod test_helpers {
    use anyhow::Result;
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
    use tempfile::TempDir;

    pub async fn create_test_database() -> Result<(SqlitePool, TempDir)> {
        let temp_dir = tempfile::tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let database_url = format!("sqlite:{}", db_path.to_string_lossy());

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok((pool, temp_dir))
    }
}

#[tokio::test]
#[serial]
async fn test_health_endpoint() {
    let (pool, _temp_dir) = test_helpers::create_test_database().await.unwrap();
    let app = create_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/health").await;
    
    response.assert_status_ok();
    
    let body: Value = response.json();
    assert_eq!(body["success"], true);
    assert_eq!(body["data"], "OK");
}

#[tokio::test]
#[serial]
async fn test_hello_endpoint() {
    let (pool, _temp_dir) = test_helpers::create_test_database().await.unwrap();
    let app = create_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/hello")
        .json(&json!({"name": "Test"}))
        .await;
    
    response.assert_status_ok();
    
    let body: Value = response.json();
    assert_eq!(body["success"], true);
    assert!(body["data"]["greeting"].as_str().unwrap().contains("Test"));
}

#[tokio::test]
#[serial]
async fn test_user_crud_operations() {
    let (pool, _temp_dir) = test_helpers::create_test_database().await.unwrap();
    let app = create_app(pool);
    let server = TestServer::new(app).unwrap();

    // Test creating a user
    let create_response = server
        .post("/api/users")
        .json(&json!({
            "name": "Test User",
            "email": "test@example.com"
        }))
        .await;
    
    create_response.assert_status_ok();
    
    let create_body: Value = create_response.json();
    assert_eq!(create_body["success"], true);
    let user_id = create_body["data"]["id"].as_str().unwrap();
    assert_eq!(create_body["data"]["name"], "Test User");
    assert_eq!(create_body["data"]["email"], "test@example.com");

    // Test getting all users
    let get_all_response = server.get("/api/users").await;
    get_all_response.assert_status_ok();
    
    let get_all_body: Value = get_all_response.json();
    assert_eq!(get_all_body["success"], true);
    assert!(get_all_body["data"].as_array().unwrap().len() >= 1);

    // Test getting a specific user
    let get_user_response = server
        .get(&format!("/api/users/{}", user_id))
        .await;
    
    get_user_response.assert_status_ok();
    
    let get_user_body: Value = get_user_response.json();
    assert_eq!(get_user_body["success"], true);
    assert_eq!(get_user_body["data"]["id"], user_id);

    // Test updating a user
    let update_response = server
        .put(&format!("/api/users/{}", user_id))
        .json(&json!({
            "name": "Updated User",
            "email": "updated@example.com"
        }))
        .await;
    
    update_response.assert_status_ok();
    
    let update_body: Value = update_response.json();
    assert_eq!(update_body["success"], true);
    assert_eq!(update_body["data"]["name"], "Updated User");
    assert_eq!(update_body["data"]["email"], "updated@example.com");

    // Test deleting a user
    let delete_response = server
        .delete(&format!("/api/users/{}", user_id))
        .await;
    
    delete_response.assert_status_ok();

    // Verify user is deleted
    let get_deleted_response = server
        .get(&format!("/api/users/{}", user_id))
        .await;
    
    get_deleted_response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
#[serial]
async fn test_project_operations() {
    let (pool, _temp_dir) = test_helpers::create_test_database().await.unwrap();
    let app = create_app(pool);
    let server = TestServer::new(app).unwrap();

    // First create a user
    let user_response = server
        .post("/api/users")
        .json(&json!({
            "name": "Project Owner",
            "email": "owner@example.com"
        }))
        .await;
    
    user_response.assert_status_ok();
    let user_body: Value = user_response.json();
    let user_id = user_body["data"]["id"].as_str().unwrap();

    // Test creating a project
    let project_response = server
        .post("/api/projects")
        .json(&json!({
            "name": "Test Project",
            "description": "A test project",
            "user_id": user_id
        }))
        .await;
    
    project_response.assert_status_ok();
    
    let project_body: Value = project_response.json();
    assert_eq!(project_body["success"], true);
    assert_eq!(project_body["data"]["name"], "Test Project");
    assert_eq!(project_body["data"]["description"], "A test project");

    // Test getting all projects
    let get_projects_response = server.get("/api/projects").await;
    get_projects_response.assert_status_ok();
    
    let get_projects_body: Value = get_projects_response.json();
    assert_eq!(get_projects_body["success"], true);
    assert!(get_projects_body["data"].as_array().unwrap().len() >= 1);
}

#[tokio::test]
#[serial]
async fn test_error_cases() {
    let (pool, _temp_dir) = test_helpers::create_test_database().await.unwrap();
    let app = create_app(pool);
    let server = TestServer::new(app).unwrap();

    // Test invalid user creation (missing email)
    let invalid_user_response = server
        .post("/api/users")
        .json(&json!({"name": "Test User"}))
        .await;
    
    invalid_user_response.assert_status(StatusCode::BAD_REQUEST);

    // Test getting non-existent user
    let non_existent_response = server
        .get("/api/users/00000000-0000-0000-0000-000000000000")
        .await;
    
    non_existent_response.assert_status(StatusCode::NOT_FOUND);

    // Test invalid UUID format
    let invalid_uuid_response = server
        .get("/api/users/invalid-uuid")
        .await;
    
    invalid_uuid_response.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn test_duplicate_email() {
    let (pool, _temp_dir) = test_helpers::create_test_database().await.unwrap();
    let app = create_app(pool);
    let server = TestServer::new(app).unwrap();

    // Create first user
    let first_user_response = server
        .post("/api/users")
        .json(&json!({
            "name": "First User",
            "email": "duplicate@example.com"
        }))
        .await;
    
    first_user_response.assert_status_ok();

    // Try to create second user with same email
    let duplicate_user_response = server
        .post("/api/users")
        .json(&json!({
            "name": "Second User",
            "email": "duplicate@example.com"
        }))
        .await;
    
    duplicate_user_response.assert_status(StatusCode::BAD_REQUEST);
}