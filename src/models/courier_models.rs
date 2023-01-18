use serde::{Deserialize, Serialize};
use diesel::Queryable;

extern crate uuid;
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Courier {
    pub is_free: bool,
    pub rating: f64,
    pub uuid: Uuid,
}

#[derive(Queryable, PartialEq, Debug)]
#[diesel(belongs_to(users))]
#[diesel(table_name = courier)]
pub struct Couriers {
    pub is_free: bool,
    pub rating: f64,
    pub uuid: Uuid,
}