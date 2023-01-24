use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PostgresPool {
   dotenv().ok();
   let database_url = env::var("DATABASE_URL").expect("no DB URL");
   println!("{}", &database_url);

   let connection = ConnectionManager::<PgConnection>::new(database_url);
   Pool::builder()
       .build(connection)
       .expect("could not build connection Pool")

}