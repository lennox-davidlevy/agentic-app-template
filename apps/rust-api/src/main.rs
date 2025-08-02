mod database;
mod errors;
mod handlers;
mod models;
mod routes;

use handlers::users::create_initial_state;
use routes::create_routes;

#[tokio::main]
async fn main() {
    println!("Starting Rust API server...");

    let state = create_initial_state();

    let app = create_routes().with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
