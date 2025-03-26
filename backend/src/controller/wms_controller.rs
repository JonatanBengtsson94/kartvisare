use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use crate::{domain::wms_details::WmsDetails, repository::repo_error::RepoError, AppState};

pub async fn get_wms_groups(
    State(state): State<AppState>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse {
    match state.wms_service.get_wms_groups(user_id).await {
        Ok(wms_groups) => Json(wms_groups).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_wms_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse {
    match state.wms_service.get_wms_by_id(id, user_id).await {
        Ok(details) => Json(details).into_response(),
        Err(RepoError::Forbidden) => StatusCode::FORBIDDEN.into_response(),
        Err(RepoError::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn add_wms(
    State(state): State<AppState>,
    Json(payload): Json<WmsDetails>,
) -> impl IntoResponse {
    match state.wms_service.add_wms(payload).await {
        Ok(wms_id) => (StatusCode::CREATED, Json(wms_id)).into_response(),
        Err(RepoError::Forbidden) => StatusCode::FORBIDDEN.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
