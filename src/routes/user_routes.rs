 use actix_web::web;
 use crate::handlers::handlers_user::*;

 pub fn config_users(conf: &mut web::ServiceConfig) {
     let scope = web::scope("/api/v1/users")
         .service(get_all_users)
         .service(get_user)
         .service(update_user)
         .service(update_password)
         .service(delete_user);
     conf.service(scope);
 }
