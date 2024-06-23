use serde::{Deserialize, Serialize};

// Struct untuk entitas User dalam domain bisnis
#[derive(Debug,Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Struct untuk representasi data dari User untuk operasi IO (Input/Output)
#[derive(Debug,Deserialize, Serialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}