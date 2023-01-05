use std::{collections::HashMap, sync::{RwLock, Arc, Mutex}};
use lazy_static::lazy_static;

use crate::{models::User, consts::EXCEPT_MAP_READ_MESSAGE};

lazy_static! {
    static ref HASHMAP: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn get_user(id: String) -> Option<User> {
    let map = (&HASHMAP)
        .lock()
        .expect(&[EXCEPT_MAP_READ_MESSAGE, "HASHMAP"].concat());

    let result = map.get(&id).cloned();

    result
}

pub fn insert_user(user: User) {
    let lock = RwLock::new(&HASHMAP);
    let mutex = lock.write().unwrap();
    
    let mut map = mutex
        .lock()
        .expect(&[EXCEPT_MAP_READ_MESSAGE, "HASHMAP"].concat());

    let user_id = (*user.user_id).to_string();
    map.insert(user_id, user);
}