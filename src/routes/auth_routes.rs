 use actix_web::{web, Scope};
 use crate::handlers::handlers_auth::*;

 pub fn auth_routes() -> Scope {
     web::scope("/")
         .route("/register", web::post().to(register))
         .route("/login", web::get().to(login))
 }
