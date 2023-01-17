use std::vec::Vec;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;

extern crate uuid;
use uuid::Uuid;

use crate::models::courier_models::Courier;
use crate::schema::schema::courier::dsl::*;


pub fn get_couriers(mut conn: PooledConnection<ConnectionManager<PgConnection>>) -> Result<Vec<Courier>, Error> {
    let result = courier.load::<Courier>(&mut conn)?;
    Ok(result)
}

pub fn get_rating(mut conn: PooledConnection<ConnectionManager<PgConnection>>, id_courier: Uuid) -> Result<Vec<f64>, Error> {
    let courier_rating: Vec<f64> = courier
        .filter(uuid.eq(id_courier))
        .select(rating)
        .get_results::<f64>(&mut conn)?;
    return Ok(courier_rating)
}

// WILL BE IMPLEMENTED AFTER ORDER SERVICE WITH gRPC
// pub fn update_rating(mut conn: PooledConnection<ConnectionManager<PgConnection>>,
//                      courier_uuid: Uuid,
//                      payload: web::Json<UpdateCourierRating>,
//                      new_rating: f64) -> Result<f64, Error> { }