pub mod response_helpers;

use actix_web::{get, HttpResponse, Responder, post, web};
use serde_json::Error;
use crate::endpoints::response_helpers::{fetch_create_user_result, fetch_get_user_result, fetch_get_users_result};
use crate::consts::EXCEPT_DEFAULT_MESSAGE;
use crate::models::UserCreateRequest;

#[get("/users")]
pub async fn get_users_route() -> impl Responder {
    fetch_get_users_result()
}

#[get("/users/{user_id}")]
pub async fn get_user_route(request: web::Path<String>) -> impl Responder {
    let user_id = request.into_inner();
    fetch_get_user_result(user_id)
}

#[post("/users/create")]
pub async fn post_user_route(body: String) -> impl Responder {
    let user_request_result: Result<UserCreateRequest, Error> = serde_json::from_str(&body);
    let valid_request: bool = match &user_request_result {
        Ok(_) => true,
        Err(_) => false
    };

    if !valid_request {
        return HttpResponse::BadRequest().finish();
    }

    let user_request = user_request_result
        .expect(&[EXCEPT_DEFAULT_MESSAGE, "user_request"].concat());

    fetch_create_user_result(user_request)
}