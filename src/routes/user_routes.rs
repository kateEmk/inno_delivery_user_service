 use actix_web::{web, Scope};
 use crate::handlers::handlers_user::*;

 pub fn user_routes() -> Scope {
     web::scope("users")
         .route("/", web::get().to(get_all_users))
         .route("/{uuid}", web::get().to(get_user))
         .route("/{uuid}", web::put().to(update_user))
         .route("/{uuid}", web::put().to(update_password))
         .route("/{uuid}", web::delete().to(delete_user))
 }
