 use actix_web::{web, Scope};
 use crate::handlers::handlers_auth::*;

 pub fn auth_routes() -> Scope {
     web::scope("/user")
         .route("/register", web::post().to(register))
         .route("/login", web::post().to(login))
 }
