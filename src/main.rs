mod handlers;
mod routes;
mod models;
mod resources;
mod schema;
mod services;
mod errors;
mod middleware;
mod tests;

extern crate serde;
extern crate serde_json;

extern crate diesel;
use std::env;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use crate::routes::user_routes::user_routes;
use crate::resources::db;
use crate::routes::auth_routes::auth_routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    let pool = db::get_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(
                web::Data::new(pool.clone()))
            .service(user_routes())
            .service(auth_routes())
            // .route("", web::get().to(user_routes()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}