
use chrono::Utc;
use sqlx::MySqlPool;
use uuid::Uuid;
use crate::domain::{models::user::{User, UserResponse}, schema::user::FilterUser};
use log::error;

pub async fn create_user_repository(db_pool: &MySqlPool, user: &User) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now();
    let updated_at = created_at;

    let query_result = sqlx::query(
        r#"INSERT INTO users (id, name, email, password, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?)"#)
        .bind(&id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(db_pool)
        .await;

    match query_result {
        Ok(_) => Ok(User {
            id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
            created_at,
            updated_at,
        }),
        Err(e) => {
            error!("Failed to execute query: {:?}", e);
            Err(e)
        }
    }
}


pub async fn get_user_repository(
    opts: Option<FilterUser>, 
    db_pool: &MySqlPool
) -> Result<Vec<UserResponse>, sqlx::Error> {
   let FilterUser { limit, page } = opts.unwrap_or_default();
    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as::<_, UserResponse>(r#"SELECT * FROM users ORDER BY id LIMIT ? OFFSET ?"#)
        .bind(limit as i32)
        .bind(offset as i32)
        .fetch_all(db_pool)
        .await;

    query_result
}
