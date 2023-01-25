mod handlers;
mod routes;
mod models;
mod resources;
mod schema;
mod services;
mod errors;
mod middleware;

extern crate diesel;
extern crate serde;
extern crate serde_json;

use std::{io::Error, env};
use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, middleware::Logger};
use dotenv::dotenv;
use crate::resources::db::establish_connection;
use crate::routes::courier_routes::config_courier;
use crate::routes::user_routes::config_users;
use crate::routes::auth_routes::config_auth;


#[actix_rt::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=debug");
    }
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(actix_web::web::Data::new(establish_connection()))
            .configure(config_auth)
            .configure(config_users)
            .configure(config_courier)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
