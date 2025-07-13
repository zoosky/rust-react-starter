#[cfg(test)]
pub mod test_helpers {
    use anyhow::Result;
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
    use tempfile::TempDir;

    pub async fn create_test_database() -> Result<(SqlitePool, TempDir)> {
        // Create a temporary directory for the test database
        let temp_dir = tempfile::tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let database_url = format!("sqlite:{}", db_path.to_string_lossy());

        // Create the database pool
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok((pool, temp_dir))
    }

    pub fn create_test_user_data() -> (String, String) {
        (
            "Test User".to_string(),
            "test@example.com".to_string(),
        )
    }

    pub fn create_test_project_data() -> (String, Option<String>) {
        (
            "Test Project".to_string(),
            Some("A test project description".to_string()),
        )
    }
}