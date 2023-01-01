use actix_web::{get, HttpResponse, Responder, post, web};
use uuid::Uuid;
use crate::data::{get_user, insert_user};
use crate::models::{User, UserCreateRequest};

#[get("/users/{user_name}")]
pub async fn get_user_route(request: web::Path<String>) -> impl Responder {
    let user_name = request.into_inner();
    let result = get_user(user_name.to_string());

    let user_found = match result {
        Some(_) => true,
        None => false
    };

    if user_found {
        let user_object = result.unwrap();
        let user_string_result = serde_json::to_string(&user_object);
        let user_string = user_string_result.unwrap();
        
        return HttpResponse::Ok().body(user_string)
    }

    HttpResponse::NotFound().body("Not found")
}

#[post("/users")]
pub async fn post_user_route(body: String) -> impl Responder {
    let user_request: UserCreateRequest = serde_json::from_str(&body).unwrap();

    let user = User {
        name: user_request.name,
        age: user_request.age,
        user_id: Uuid::new_v4().to_string()
    };

    insert_user(user);
    HttpResponse::Ok().body("")
}