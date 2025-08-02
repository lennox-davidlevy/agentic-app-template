use axum::{response::Json};
use chrono::Utc;

use crate::models::{ApiResponse, HealthResponse};

pub async fn hello_world() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello from Rust!".to_string(),
        status: "succes".to_string(),
    })
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
    })
}
