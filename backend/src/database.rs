use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

pub async fn create_database_pool() -> Result<SqlitePool> {
    // Create data directory if it doesn't exist
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir)?;
    }

    let database_url = "sqlite:data/app.db";
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::tempdir;

    async fn create_test_database() -> anyhow::Result<(sqlx::SqlitePool, tempfile::TempDir)> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let database_url = format!("sqlite:{}", db_path.to_string_lossy());

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok((pool, temp_dir))
    }

    #[tokio::test]
    #[serial]
    async fn test_create_database_pool() {
        let result = create_database_pool().await;
        assert!(result.is_ok());
        
        let pool = result.unwrap();
        
        // Test that we can execute a simple query
        let result = sqlx::query("SELECT 1").fetch_one(&pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_test_database_creation() {
        let result = create_test_database().await;
        assert!(result.is_ok());
        
        let (pool, _temp_dir) = result.unwrap();
        
        // Test that migrations ran successfully by checking tables exist
        let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='users'")
            .fetch_one(&pool)
            .await;
        assert!(result.is_ok());
        
        let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='projects'")
            .fetch_one(&pool)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_database_constraints() {
        let (pool, _temp_dir) = create_test_database().await.unwrap();
        
        // Test unique email constraint
        let user_id1 = uuid::Uuid::new_v4().to_string();
        let user_id2 = uuid::Uuid::new_v4().to_string();
        
        // Insert first user
        let result = sqlx::query(
            "INSERT INTO users (id, name, email) VALUES (?, ?, ?)"
        )
        .bind(&user_id1)
        .bind("User 1")
        .bind("test@example.com")
        .execute(&pool)
        .await;
        assert!(result.is_ok());
        
        // Try to insert second user with same email (should fail)
        let result = sqlx::query(
            "INSERT INTO users (id, name, email) VALUES (?, ?, ?)"
        )
        .bind(&user_id2)
        .bind("User 2")
        .bind("test@example.com")
        .execute(&pool)
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[serial]
    async fn test_foreign_key_constraints() {
        let (pool, _temp_dir) = create_test_database().await.unwrap();
        
        // Enable foreign key constraints
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();
        
        let project_id = uuid::Uuid::new_v4().to_string();
        let non_existent_user_id = uuid::Uuid::new_v4().to_string();
        
        // Try to insert project with non-existent user_id (should fail)
        let result = sqlx::query(
            "INSERT INTO projects (id, name, user_id) VALUES (?, ?, ?)"
        )
        .bind(&project_id)
        .bind("Test Project")
        .bind(&non_existent_user_id)
        .execute(&pool)
        .await;
        assert!(result.is_err());
    }
}