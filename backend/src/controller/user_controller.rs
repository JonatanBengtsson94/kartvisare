use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::AppState;

pub async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    match state.user_service.get_users().await {
        Ok(users) => Json(users).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
