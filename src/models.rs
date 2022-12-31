use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub age: i32
}
