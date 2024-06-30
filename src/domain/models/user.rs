use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Struct untuk entitas User dalam domain bisnis
#[derive(Debug,Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug,Deserialize, Serialize, sqlx::FromRow)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Struct untuk representasi data dari User untuk operasi IO (Input/Output)
#[derive(Debug,Deserialize, Serialize)]
pub struct UserDto {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<Utc>>, 
}


impl UserDto {
   pub fn into_user(self) -> User {
        User {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            email: self.email,
            password: self.password, 
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
