use actix_web::{HttpServer, App};
use crate::endpoints::{get_user_route, post_user_route};

mod data;
mod endpoints;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user_route)
            .service(post_user_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}