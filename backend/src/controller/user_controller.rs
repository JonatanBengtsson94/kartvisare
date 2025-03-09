use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    domain::user::User, repository::user_repository::UserRepository,
    service::user_service::UserService,
};

pub async fn get_users<R: UserRepository>(
    State(service): State<UserService<R>>,
) -> impl IntoResponse {
    match service.get_users().await {
        Ok(users) => Json(users).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
