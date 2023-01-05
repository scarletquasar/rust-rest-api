use actix_web::HttpResponse;
use uuid::Uuid;

use crate::{
    consts::EXCEPT_DEFAULT_MESSAGE,
    data::{insert_user, get_user, get_users}, 
    models::{UserCreateRequest, User, UserCreateResponse}
};

pub fn fetch_create_user_result(user_request: UserCreateRequest) -> HttpResponse {
    let user_id = Uuid::new_v4().to_string();

    let user = User {
        name: user_request.name,
        age: user_request.age,
        user_id: user_id.to_string()
    };

    let response = UserCreateResponse {
        success: true,
        user_id: user_id.to_string()
    };

    let response_string = serde_json::to_string(&response)
        .expect(&[EXCEPT_DEFAULT_MESSAGE, "response_string"].concat());

    insert_user(user);
    HttpResponse::Ok().body(response_string)
}

pub fn fetch_get_users_result() -> HttpResponse {
    let users = get_users();

    let response_string = serde_json::to_string(&users)
        .expect(&[EXCEPT_DEFAULT_MESSAGE, "response_string"].concat());

    HttpResponse::Ok().body(response_string)
}

pub fn fetch_get_user_result(user_id: String) -> HttpResponse {
    let result = get_user(user_id.to_string());

    let user_found = match result {
        Some(_) => true,
        None => false
    };

    if user_found {
        let user_object = result
            .expect(&[EXCEPT_DEFAULT_MESSAGE, "user_object"].concat());

        let user_string_result = serde_json::to_string(&user_object);

        let user_string = user_string_result
            .expect(&[EXCEPT_DEFAULT_MESSAGE, "user_string"].concat());
        
        return HttpResponse::Ok().body(user_string)
    }

    HttpResponse::NotFound().body("Not found")
}