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
use std::{io::Error, env};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use crate::routes::user_routes::user_routes;
use crate::resources::db::establish_connection;
use crate::routes::auth_routes::auth_routes;


#[actix_rt::main]
async fn main() -> Result<(), Error> {

    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env::set_var("RUST_BACKTRACE", "1");

    println!("Start server localhost:8080");

    HttpServer::new(move || {

        App::new()
            // .app_data(establish_connection())
            .app_data(actix_web::web::Data::new(establish_connection()))
            .service(user_routes())
            .service(auth_routes())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}