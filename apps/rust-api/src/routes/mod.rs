use crate::handlers::health::{health_check, hello_world};
use crate::handlers::users::{AppState, create_user, get_user_by_id, get_users};
use axum::{Router, routing::get};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/{id}", get(get_user_by_id))
}
