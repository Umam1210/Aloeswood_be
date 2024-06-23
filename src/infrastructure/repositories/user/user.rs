use chrono::Utc;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::domain::models::user::{User, UserDto};


pub async fn create_user_repository(db_pool: &MySqlPool, user: &UserDto) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now();
    let updated_at = created_at;

    let query_result = sqlx::query(
        r#"INSERT INTO users (id, name, email, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?)"#)
        .bind(&id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(db_pool)
        .await?;

    if query_result.rows_affected() == 1 {
        Ok(User {
            id,
            name: user.name.clone(),
            password: String::new(), // Placeholder for now
            email: user.email.clone(),
            created_at,
            updated_at,
        })
    } else {
        Err(sqlx::Error::RowNotFound) // Or handle accordingly
    }
}