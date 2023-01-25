 use actix_web::web;
 use crate::handlers::handlers_auth::*;

 pub fn config_auth(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(register)
        .service(login);
    conf.service(scope);
}