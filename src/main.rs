use actix_web::{HttpServer, App};
use endpoints::get_users_route;
use crate::endpoints::{get_user_route, post_user_route};

mod data;
mod endpoints;
mod models;
mod consts;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(get_users_route)
            .service(get_user_route)
            .service(post_user_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}