use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use thiserror::Error;
use validator::ValidationErrors;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Validation failed: {0}")]
    ValidationError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        
        Json(serde_json::json!({
            "error": message,
            "status": status.as_u16()
        }))
        .into_response()
    }
}

// Convert validation errors to our ApiError
impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> Self {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!("{}: {}", field, error.message.as_ref().unwrap_or(&"Invalid value".into()))
                })
            })
            .collect();
        
        ApiError::ValidationError(error_messages.join(", "))
    }
}
