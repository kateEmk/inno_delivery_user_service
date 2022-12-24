 use actix_web::{web, Scope};
 use crate::auth::middleware::*;
 use crate::handlers::handlers::*;

 pub fn register_urls() -> Scope {
     web::scope("/user")
            .route("/login", web::get().to(login))
            .route("/logout", web::get().to(logout))
            .route("/users", web::get().to(get_users))

            // .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            // .route("/users", web::post().to(handlers::add_user))
            // .route("/users/{id}", web::delete().to(handlers::delete_user))
 }
