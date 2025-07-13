use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            email,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let name = "Test User".to_string();
        let email = "test@example.com".to_string();
        
        let user = User::new(name.clone(), email.clone());
        
        assert_eq!(user.name, name);
        assert_eq!(user.email, email);
        assert!(!user.id.is_empty());
        assert!(Uuid::parse_str(&user.id).is_ok());
        assert_eq!(user.created_at, user.updated_at);
    }

    #[test]
    fn test_user_serialization() {
        let user = User::new("Test User".to_string(), "test@example.com".to_string());
        
        // Test serialization to JSON
        let json = serde_json::to_string(&user);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("Test User"));
        assert!(json_str.contains("test@example.com"));
    }

    #[test]
    fn test_create_user_request_deserialization() {
        let json = r#"{"name": "Test User", "email": "test@example.com"}"#;
        
        let request: Result<CreateUserRequest, _> = serde_json::from_str(json);
        assert!(request.is_ok());
        
        let request = request.unwrap();
        assert_eq!(request.name, "Test User");
        assert_eq!(request.email, "test@example.com");
    }

    #[test]
    fn test_update_user_request_deserialization() {
        // Test with both fields
        let json = r#"{"name": "Updated User", "email": "updated@example.com"}"#;
        let request: Result<UpdateUserRequest, _> = serde_json::from_str(json);
        assert!(request.is_ok());
        
        let request = request.unwrap();
        assert_eq!(request.name, Some("Updated User".to_string()));
        assert_eq!(request.email, Some("updated@example.com".to_string()));

        // Test with only name
        let json = r#"{"name": "Updated User"}"#;
        let request: Result<UpdateUserRequest, _> = serde_json::from_str(json);
        assert!(request.is_ok());
        
        let request = request.unwrap();
        assert_eq!(request.name, Some("Updated User".to_string()));
        assert_eq!(request.email, None);

        // Test with empty object
        let json = r#"{}"#;
        let request: Result<UpdateUserRequest, _> = serde_json::from_str(json);
        assert!(request.is_ok());
        
        let request = request.unwrap();
        assert_eq!(request.name, None);
        assert_eq!(request.email, None);
    }

    #[test]
    fn test_user_id_uniqueness() {
        let user1 = User::new("User 1".to_string(), "user1@example.com".to_string());
        let user2 = User::new("User 2".to_string(), "user2@example.com".to_string());
        
        assert_ne!(user1.id, user2.id);
    }
}