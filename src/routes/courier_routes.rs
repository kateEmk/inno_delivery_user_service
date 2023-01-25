 use actix_web::web;
 use crate::handlers::handlers_courier::*;

  pub fn config_courier(conf: &mut web::ServiceConfig) {
     let scope = web::scope("/api/v1/couriers")
         .service(get_all_couriers)
         .service(get_courier_rating);
     conf.service(scope);
 }
