use axum::{routing::get, Router};

mod controller;
mod model;
mod repository;
mod service;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(|| async { "Hello World" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_hello).await.unwrap();
}
