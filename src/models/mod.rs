use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub name: String,
    pub age: i32
}

#[derive(Serialize)]
pub struct UserCreateResponse {
    pub success: bool,
    pub user_id: String
}

#[derive(Clone, Serialize)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub age: i32
}