use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct User {
    pub name: String,
    pub age: i32
}

pub struct UserGetRequest {
    pub name: String
}
