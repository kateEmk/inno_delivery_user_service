 use actix_web::{web, Scope};
 use crate::handlers::handlers_courier::*;

 pub fn courier_routes() -> Scope {
     web::scope("couriers")
         .route("/", web::get().to(get_all_couriers))
         .route("/{uuid}", web::get().to(get_courier_rating))
 }
