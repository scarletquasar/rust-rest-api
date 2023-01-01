use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub age: i32
}

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub name: String,
    pub age: i32
}