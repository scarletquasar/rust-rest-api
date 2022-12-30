use std::{collections::HashMap, sync::{RwLock, Arc, Mutex}};
use lazy_static::lazy_static;

use crate::models::User;

lazy_static! {
    static ref HASHMAP: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn get_user(name: String) -> Option<User> {
    let map = (&HASHMAP).lock().unwrap();
    let result = map.get(&name).cloned();

    result
}

pub fn insert_user(user: User) {
    let lock = RwLock::new(&HASHMAP);
    let mutex = lock.write().unwrap();
    
    let mut map = mutex.lock().unwrap();
    let user_name = (*user.name).to_string();
    map.insert(user_name, user);
}

