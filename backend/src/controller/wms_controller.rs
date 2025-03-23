use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{domain::wms_details::WmsDetails, AppState};

pub async fn get_wms_groups(
    State(state): State<AppState>,
    request: Request<Body>,
) -> impl IntoResponse {
    let user_id = request.extensions().get::<i32>();
    match user_id {
        Some(id) => match state.wms_service.get_wms_groups(*id).await {
            Ok(wms_groups) => Json(wms_groups).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}

pub async fn get_wms_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    request: Request<Body>,
) -> impl IntoResponse {
    let user_id = request.extensions().get::<i32>();
    match user_id {
        Some(u_id) => match state.wms_service.get_wms_by_id(id, *u_id).await {
            Ok(Some(details)) => Json(details).into_response(),
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        None => StatusCode::UNAUTHORIZED.into_response(),
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
