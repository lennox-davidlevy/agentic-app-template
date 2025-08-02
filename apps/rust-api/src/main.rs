use axum::{
    Router,
    routing::get,
};

mod errors;
mod models;
mod handlers;

use handlers::{hello_world, health_check, get_users, get_user_by_id};

#[tokio::main]
async fn main() {
    println!("Starting Rust API server...");

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .route("/api/users", get(get_users))
        .route("/api/users/{id}", get(get_user_by_id));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");
    println!("Try: /health and /api/users");

    axum::serve(listener, app).await.unwrap();
}
