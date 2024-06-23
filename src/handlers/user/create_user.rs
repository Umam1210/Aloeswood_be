use std::sync::Arc;

use axum::{
    http::StatusCode, response::{IntoResponse, Response}, Extension, Json
};
use sqlx::MySqlPool;

use crate::{domain::models::user::UserDto, infrastructure::repositories::user};




pub async fn create_user_handler(
    Extension(db_pool): Extension<Arc<MySqlPool>>,
    Json(user): Json<UserDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match user::user::create_user_repository(&db_pool, &user).await {
        Ok(created_user) => {
            let user_response = serde_json::json!({
                "status": "success",
                "data": {
                    "user": {
                        "id": created_user.id,
                        "name": created_user.name,
                        "email": created_user.email,
                        "created_at": created_user.created_at,
                        "updated_at": created_user.updated_at,
                    }
                }
            });
            Ok(Response::new(Json(user_response)))
        },
        Err(_) => {
            let error_response = serde_json::json!!({
                "status": "error",
                "message": "Failed to create user",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}