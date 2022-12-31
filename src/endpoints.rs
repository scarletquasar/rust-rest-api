use actix_web::{get, HttpResponse, Responder, post, web};
use crate::data::{get_user, insert_user};
use crate::models::User;

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
    let user: User = serde_json::from_str(&body).unwrap();

    insert_user(user);
    HttpResponse::Ok().body("")
}