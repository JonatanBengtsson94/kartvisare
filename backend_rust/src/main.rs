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
    let pool = PgPool::connect("postgres://kartvisare:kartvisare@localhost:5432/kartvisare")
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
