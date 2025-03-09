use std::env;

use axum::{routing::get, Router};
use controller::user_controller;
use repository::user_repository::PostgresUserRepository;
use service::user_service::UserService;
use sqlx::PgPool;

mod controller;
mod domain;
mod repository;
mod service;

#[tokio::main]
async fn main() {
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

    let repository = PostgresUserRepository::new(pool);
    let service = UserService::new(repository);

    let app: Router = Router::new()
        .route("/users", get(user_controller::get_users))
        .with_state(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
