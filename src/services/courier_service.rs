use std::vec::Vec;
use actix_web::web;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use serde_json::{json, Value};

extern crate uuid;
use uuid::Uuid;

use crate::models::models::{Courier, UpdateCourierRating};
use crate::schema::schema::courier::dsl::*;
use crate::schema::schema::courier::*;


pub fn get_couriers(mut conn: PooledConnection<ConnectionManager<PgConnection>>) -> Result<Vec<Courier>, Error> {
    let result = courier.load::<Courier>(&mut conn)?;
    Ok(result)
}

pub fn get_rating(mut conn: PooledConnection<ConnectionManager<PgConnection>>, id_courier: Uuid) -> Result<Value, Error> {
    let courier_rating = courier
        .filter(uuid.eq(id_courier))
        .select(rating)
        .first::<Courier>(&mut conn)?;
    return Ok(json!(courier_rating))
}

// WILL BE IMPLEMENTED AFTER ORDER SERVICE WITH gRPC
// pub fn update_rating(mut conn: PooledConnection<ConnectionManager<PgConnection>>,
//                      courier_uuid: Uuid,
//                      payload: web::Json<UpdateCourierRating>,
//                      new_rating: f64) -> Result<f64, Error> {
//     let courier_updated = diesel::update(courier
//         .find(courier_uuid))
//         .set(payload.rating.eq(&new_rating))
//         .get_result::<UpdateCourierRating>(&mut conn)?;
//
//     return Ok(courier_updated.rating)
// }