use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<User>,
    pub total: usize
}
