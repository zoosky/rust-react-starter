use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Project {
    pub fn new(name: String, description: Option<String>, user_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            user_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation_with_description() {
        let name = "Test Project".to_string();
        let description = Some("A test project description".to_string());
        let user_id = Uuid::new_v4().to_string();
        
        let project = Project::new(name.clone(), description.clone(), user_id.clone());
        
        assert_eq!(project.name, name);
        assert_eq!(project.description, description);
        assert_eq!(project.user_id, user_id);
        assert!(!project.id.is_empty());
        assert!(Uuid::parse_str(&project.id).is_ok());
        assert_eq!(project.created_at, project.updated_at);
    }

    #[test]
    fn test_project_creation_without_description() {
        let name = "Test Project".to_string();
        let user_id = Uuid::new_v4().to_string();
        
        let project = Project::new(name.clone(), None, user_id.clone());
        
        assert_eq!(project.name, name);
        assert_eq!(project.description, None);
        assert_eq!(project.user_id, user_id);
    }

    #[test]
    fn test_project_serialization() {
        let user_id = Uuid::new_v4().to_string();
        let project = Project::new(
            "Test Project".to_string(),
            Some("Description".to_string()),
            user_id,
        );
        
        // Test serialization to JSON
        let json = serde_json::to_string(&project);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("Test Project"));
        assert!(json_str.contains("Description"));
    }

    #[test]
    fn test_create_project_request_deserialization() {
        let user_id = Uuid::new_v4().to_string();
        let json = format!(
            r#"{{"name": "Test Project", "description": "Test Description", "user_id": "{}"}}"#,
            user_id
        );
        
        let request: Result<CreateProjectRequest, _> = serde_json::from_str(&json);
        assert!(request.is_ok());
        
        let request = request.unwrap();
        assert_eq!(request.name, "Test Project");
        assert_eq!(request.description, Some("Test Description".to_string()));
        assert_eq!(request.user_id, user_id);
    }

    #[test]
    fn test_create_project_request_without_description() {
        let user_id = Uuid::new_v4().to_string();
        let json = format!(
            r#"{{"name": "Test Project", "user_id": "{}"}}"#,
            user_id
        );
        
        let request: Result<CreateProjectRequest, _> = serde_json::from_str(&json);
        assert!(request.is_ok());
        
        let request = request.unwrap();
        assert_eq!(request.name, "Test Project");
        assert_eq!(request.description, None);
        assert_eq!(request.user_id, user_id);
    }

    #[test]
    fn test_update_project_request_deserialization() {
        // Test with both fields
        let json = r#"{"name": "Updated Project", "description": "Updated Description"}"#;
        let request: Result<UpdateProjectRequest, _> = serde_json::from_str(json);
        assert!(request.is_ok());
        
        let request = request.unwrap();
        assert_eq!(request.name, Some("Updated Project".to_string()));
        assert_eq!(request.description, Some("Updated Description".to_string()));

        // Test with only name
        let json = r#"{"name": "Updated Project"}"#;
        let request: Result<UpdateProjectRequest, _> = serde_json::from_str(json);
        assert!(request.is_ok());
        
        let request = request.unwrap();
        assert_eq!(request.name, Some("Updated Project".to_string()));
        assert_eq!(request.description, None);

        // Test with empty object
        let json = r#"{}"#;
        let request: Result<UpdateProjectRequest, _> = serde_json::from_str(json);
        assert!(request.is_ok());
        
        let request = request.unwrap();
        assert_eq!(request.name, None);
        assert_eq!(request.description, None);
    }

    #[test]
    fn test_project_id_uniqueness() {
        let user_id = Uuid::new_v4().to_string();
        let project1 = Project::new(
            "Project 1".to_string(),
            None,
            user_id.clone(),
        );
        let project2 = Project::new(
            "Project 2".to_string(),
            None,
            user_id,
        );
        
        assert_ne!(project1.id, project2.id);
    }
}