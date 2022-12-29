 use actix_web::{web, Scope};
 use crate::handlers::handlers_user::*;

 pub fn user_routes() -> Scope {
     web::scope("users")
         .route("/", web::get().to(get_all_users))
         .route("/{id}", web::get().to(get_user))
         .route("/{id}", web::put().to(update_user))
         .route("/{id}", web::delete().to(delete_user))

        // .route("/users", web::post().to(handlers::add_user))
 }
