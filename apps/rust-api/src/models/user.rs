use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<User>,
    pub total: usize,
}

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Name must be between 1 and 50 characters"
    ))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub message: String,
}
