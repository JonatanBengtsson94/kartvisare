use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{domain::wms_details::WmsDetails, AppState};

pub async fn get_wms_summaries(State(state): State<AppState>) -> impl IntoResponse {
    match state.wms_service.get_wms_summaries().await {
        Ok(summaries) => Json(summaries).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_wms_details(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.wms_service.get_wms_details(id).await {
        Ok(Some(details)) => Json(details).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn add_wms(
    State(state): State<AppState>,
    Json(payload): Json<WmsDetails>,
) -> impl IntoResponse {
    match state.wms_service.add_wms(payload).await {
        Ok(wms_id) => (StatusCode::CREATED, Json(wms_id)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
