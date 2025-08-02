mod errors;
mod handlers;
mod models;
mod routes;

use routes::create_routes;

#[tokio::main]
async fn main() {
    println!("Starting Rust API server...");

    let app = create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");
    println!("Try: /health and /api/users");

    axum::serve(listener, app).await.unwrap();
}
