use axum::{Router, routing::get};

use crate::handlers::{get_user_by_id, get_users, health_check, hello_world};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .route("/api/users", get(get_users))
        .route("/api/users/{id}", get(get_user_by_id))
}
