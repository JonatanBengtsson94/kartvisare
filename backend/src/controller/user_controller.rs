use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;

use crate::{
    domain::{session::Session, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct AddUserToGroupPayload {
    user_id: i32,
    group_id: i32,
}

pub async fn get_users(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
) -> impl IntoResponse {
    if !session.is_admin {
        return StatusCode::FORBIDDEN.into_response();
    }
    match state.user_service.get_users().await {
        Ok(users) => Json(users).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn add_user(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    if !session.is_admin {
        return StatusCode::FORBIDDEN.into_response();
    }
    match state.user_service.add_user(payload).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn add_user_to_group(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
    Json(payload): Json<AddUserToGroupPayload>,
) -> impl IntoResponse {
    if !session.is_admin {
        return StatusCode::FORBIDDEN.into_response();
    }
    match state
        .user_service
        .add_user_to_group(payload.user_id, payload.group_id)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn add_group(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
    Json(payload): Json<String>,
) -> impl IntoResponse {
    if !session.is_admin {
        return StatusCode::FORBIDDEN.into_response();
    }
    match state.user_service.add_group(payload).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
