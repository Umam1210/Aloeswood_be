use std::sync::Arc;

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use hyper::StatusCode;

use crate::{domain:: schema::user::FilterUser, infrastructure::repositories::user::user::get_user_repository, AppState};


pub async fn get_user_handler(
    State(data): State<Arc<AppState>>,
    Query(opts): Query<FilterUser>,
) -> impl IntoResponse {
    match get_user_repository(Some(opts), &data.db).await {
        Ok(users) => {
            let user_response = serde_json::json!({
                "status": "success",
                "data": users,
            });
            (StatusCode::OK, Json(user_response))
        },
        Err(e) => {
            log::error!("Failed to get users: {:?}", e);
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Failed to get users",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}
