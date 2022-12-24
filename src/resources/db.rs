use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;


pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PostgresPool {
   dotenv().ok();
   let url = env::var("DATABASE_URL").expect("no DB URL");
   let connection = ConnectionManager::<PgConnection>::new(url);
   Pool::builder()
       .build(connection)
       .expect("could not build connection pool")
}