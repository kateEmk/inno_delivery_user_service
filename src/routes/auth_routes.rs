 use actix_web::{web, Scope};
 use crate::handlers::handlers_user::*;

 pub fn auth_routes() -> Scope {
     web::scope("/user")
            .route("/login", web::get().to(create_user))
            //.route("/logout", web::get().to(logout))
 }