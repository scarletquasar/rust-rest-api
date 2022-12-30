use actix_web::{get, HttpResponse, Responder, post, web};
use crate::data::{get_user, insert_user};
use crate::models::{User, UserGetRequest};

#[get("/users")]
pub async fn get_user_route(data: web::Data<UserGetRequest>) -> impl Responder {
    let user = get_user(data.name.to_string());

    return match user {
        Some(x) => HttpResponse::Ok().body(x.name.to_string()),
        None => HttpResponse::NotFound().body("Not found")
    }
}

#[post("/users")]
pub async fn post_user_route(body: String) -> impl Responder {
    let user: User = serde_json::from_str(&body).unwrap();

    insert_user(user);
    HttpResponse::Ok().body("")
}