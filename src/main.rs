mod handlers;
mod routes;
mod models;
mod resources;
mod schema;
mod services;
mod errors;
mod middleware;
mod custom_layer;

extern crate diesel;
extern crate serde;
extern crate serde_json;

use std::{io::Error, env};
use actix_web::{get, App, HttpServer, Responder, middleware::Logger, dev::Service};
use dotenv::dotenv;
use crate::resources::db::establish_connection;
use crate::routes::courier_routes::courier_routes;
use crate::routes::user_routes::user_routes;
use crate::routes::auth_routes::auth_routes;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use custom_layer::CustomLayer;
use actix_web_opentelemetry::RequestTracing;

#[get("/")]
async fn hello() -> impl Responder {
    format!("Inno-delivery service")
}

fn configure_tracing() {
    let tracing_sub = tracing_subscriber::registry()
        // .with(tracing_subscriber::fmt::layer().compact());
        .with(CustomLayer);
    tracing::subscriber::set_global_default(tracing_sub).expect("configuring tracing");
}

#[actix_rt::main]
async fn main() -> Result<(), Error> {

    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");
    configure_tracing();

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                log::info!("hello from middleware!");
                srv.call(req)
            })
            .app_data(actix_web::web::Data::new(establish_connection()))
            .wrap(TracingLogger::default())
            .wrap(RequestTracing::new())
            .wrap(Logger::default())
            .service(auth_routes())
            .service(user_routes())
            .service(courier_routes())
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}