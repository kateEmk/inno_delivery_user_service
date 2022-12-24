mod handlers;
mod api;
mod models;
mod resources;
mod schemas;
mod services;
mod errors;
mod auth;
pub mod config;

extern crate diesel;
use std::env;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use crate::api::urls::register_urls;
use crate::resources::db;


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
            .route("", web::get().to(register_urls()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}