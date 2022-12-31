use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub age: i32
}
