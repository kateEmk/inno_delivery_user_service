use serde::Serialize;
use diesel::{Insertable, Queryable};
use crate::schema::schema::courier;

extern crate uuid;
use uuid::Uuid;


#[derive(Queryable, PartialEq, Serialize, Debug, Insertable)]
#[diesel(belongs_to(users))]
#[diesel(table_name = courier)]
pub struct Couriers {
    pub is_free: bool,
    pub rating: f64,
    pub uuid: Uuid,
}