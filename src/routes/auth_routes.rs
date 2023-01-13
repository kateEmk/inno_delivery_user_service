 use actix_web::{web, Scope};
 use crate::handlers::handlers_auth::*;

 pub fn auth_routes() -> Scope {
     web::scope("/user")
         .route("/register", web::get().to(register))
         // .route("register")
 }





// use crate::db::db::{Connection, IConnection};
// use crate::models::auth::{Login, Register};
// // use crate::repositories::user_repository::{IUserRepository, UserRepository};
// use actix_web::http::StatusCode;
// use actix_web::{post, get, web, HttpRequest, HttpResponse};
//  use crate::resources::db::PostgresPool;
//
//
//  #[post("/login")]
// async fn login(user: web::Json<Login>) -> HttpResponse {
//      let _connection = PostgresPool.get().unwrap();
//      let _repository: UserRepository = UserRepository {
//          connection: _connection.init(),
//      };
//      let proc = _repository.login(user.into_inner());
//
//      match proc {
//          Ok(_) => HttpResponse::Ok().json(proc.unwrap()),
//          Err(_) => HttpResponse::Ok()
//              .status(StatusCode::from_u16(401).unwrap())
//              .json(proc.unwrap_err()),
//      }
// }
//
// #[post("/register")]
// async fn register(user: web::Json<Register>) -> HttpResponse {
//     let _connection = PostgresPool.get().unwrap();
//     let _repository: UserRepository = UserRepository {
//         connection: _connection.init(),
//     };
//     HttpResponse::Ok().json(_repository.register(user.into_inner()))
// }