use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};
// use uuid::Uuid;
use crate::schema::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    // pub uuid: Uuid,
    pub first_name: String,
    pub address: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub role: String,       // USER / COURIER / ADMIN
    pub is_blocked: bool,
    pub is_deleted: bool,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime
}

pub enum Roles {
    User(String),
    Courier(String),
    Admin(String)
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Courier {
    // pub uuid: Uuid,
    pub is_free: bool,
    pub rating: f64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct Users {
    // pub uuid: Uuid,
    pub first_name: String,
    pub address: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub is_blocked: bool,
    pub is_deleted: bool,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[diesel(belongs_to(users))]
#[diesel(table_name = courier)]
pub struct Couriers {
    // pub uuid: Uuid,
    pub is_free: bool,
    pub rating: f64,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct CreateNewUser {
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String
}


#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = users)]
#[derive(AsChangeset)]
pub struct UpdateUserProfile {
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String
}


impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
           .content_type(ContentType::json())
           .body(res_body)
    }
}
