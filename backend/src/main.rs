use std::env;

use dotenv::dotenv;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use controller::{user_controller, wms_controller};
use middleware::auth::auth_middleware;
use repository::{user_repository::PostgresUserRepository, wms_repository::PostgresWmsRepository};
use service::{idp_service::MockIdpService, user_service::UserService, wms_service::WmsService};
use sqlx::PgPool;

mod controller;
mod domain;
mod middleware;
mod repository;
mod service;

#[derive(Clone)]
struct AppState {
    user_service: UserService<PostgresUserRepository>,
    wms_service: WmsService<PostgresWmsRepository>,
    idp_service: MockIdpService,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_host = env::var("DB_HOST").expect("DB_HOST must be set in the environment");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set in the environment");
    let db_user = env::var("DB_USER").expect("DB_USER must be set in the environment");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in the environment");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set in the environment");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let user_repository = PostgresUserRepository::new(pool.clone());
    let user_service = UserService::new(user_repository);

    let wms_repository = PostgresWmsRepository::new(pool.clone());
    let wms_service = WmsService::new(wms_repository);

    let idp_service = MockIdpService::new();

    let app_state = AppState {
        user_service,
        wms_service,
        idp_service,
    };

    let app: Router = Router::new()
        .route("/users", get(user_controller::get_users))
        .route("/wms", get(wms_controller::get_wms_groups))
        .route("/wms", post(wms_controller::add_wms))
        .route("/wms/{id}", get(wms_controller::get_wms_by_id))
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware::<Body>,
        ))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
