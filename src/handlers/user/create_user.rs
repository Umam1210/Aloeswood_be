use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use std::sync::Arc;

use crate::{domain::models::user:: UserDto, infrastructure::repositories::user::user::create_user_repository, AppState};

pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(user_dto): Json<UserDto>,
) -> impl IntoResponse {
    let user = user_dto.into_user();
    
    match create_user_repository(&data.db, &user).await {
        Ok(created_user) => {
            let user_response = serde_json::json!({
                "status": "success",
                "data": {
                    "user": {
                        "id": created_user.id,
                        "name": created_user.name,
                        "email": created_user.email
                    }
                }
            });
            (StatusCode::OK, Json(user_response))
        },
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Failed to create user",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}