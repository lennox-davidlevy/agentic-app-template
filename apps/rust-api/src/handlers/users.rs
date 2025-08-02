use crate::errors::ApiError;
use crate::models::{CreateUserRequest, CreateUserResponse, User, UsersResponse};
use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use validator::Validate;

pub type AppState = Arc<Mutex<Vec<User>>>;

pub fn create_initial_state() -> AppState {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 1,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];
    Arc::new(Mutex::new(users))
}

pub async fn get_users(State(state): State<AppState>) -> Json<UsersResponse> {
    let users = state.lock().await.clone();
    let total = users.len();
    Json(UsersResponse { users, total })
}

pub async fn get_user_by_id(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<User>, ApiError> {
    let users = state.lock().await;

    match users.iter().find(|user| user.id == id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(ApiError::NotFound(format!("User with id {id} not found"))),
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, ApiError> {
    payload.validate()?;

    let mut users = state.lock().await;

    if users.iter().any(|user| user.email == payload.email) {
        return Err(ApiError::BadRequest("Email already exists".to_string()));
    }

    let new_id = (users.len() + 1) as u32;

    let new_user = User {
        id: new_id,
        name: payload.name.clone(),
        email: payload.email.clone(),
    };

    users.push(new_user.clone());

    Ok(Json(CreateUserResponse {
        id: new_user.id,
        name: new_user.name,
        email: new_user.email,
        message: "User created successfully".to_string(),
    }))
}
