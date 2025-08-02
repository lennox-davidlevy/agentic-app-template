use axum::{extract::Path, response::Json};
use chrono::Utc;

use crate::errors::ApiError;
use crate::models::{ApiResponse, HealthResponse, User, UsersResponse};

pub async fn hello_world() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello from rust!".to_string(),
        status: "success".to_string(),
    })
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
    })
}

pub async fn get_users() -> Json<UsersResponse> {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];
    let total = users.len();
    Json(UsersResponse { users, total })
}

pub async fn get_user_by_id(Path(id): Path<u32>) -> Result<Json<User>, ApiError> {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    match users.into_iter().find(|user| user.id == id) {
        Some(user) => Ok(Json(user)),
        None => Err(ApiError::NotFound(format!("User with id {id} not found"))),
    }
}
