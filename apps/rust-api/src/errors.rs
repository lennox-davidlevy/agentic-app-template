use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        Json(serde_json::json!({
            "error": message,
            "status": status.as_u16()
        }))
        .into_response()
    }
}
