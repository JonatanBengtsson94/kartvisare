use crate::service::idp_service::IdpService;
use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::AppState;

pub async fn auth_middleware<B>(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let cookies = request.headers().get("Cookie");

    let session_id = cookies
        .and_then(|cookies| cookies.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str.split(";").find_map(|cookie| {
                if cookie.trim().starts_with("X-Session-ID=") {
                    Some(cookie.trim().split("=").nth(1).unwrap().to_string())
                } else {
                    None
                }
            })
        });

    let session;

    if let Some(session_id) = session_id {
        session = state
            .session_service
            .get_session(&session_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        let auth_header = request
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(token) => token,
            None => return Err(StatusCode::UNAUTHORIZED),
        };

        let user_id = state
            .idp_service
            .validate_token(token)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let is_admin = state
            .user_service
            .is_admin(user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        session = state
            .session_service
            .create_session(user_id, is_admin)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    let cookie_value = format!(
        "X-Session-ID={}; Path=/; HttpOnly; Secure; SameSite=None",
        session.session_id
    );
    request.extensions_mut().insert(session);

    let mut response = next.run(request).await;

    response
        .headers_mut()
        .insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());
    Ok(response)
}
