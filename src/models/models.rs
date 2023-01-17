use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};
use chrono::NaiveDateTime;
use crate::schema::schema::{users, courier};

extern crate uuid;
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    pub first_name: String,
    pub address: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub role: String,       // USER / COURIER / ADMIN
    pub is_blocked: bool,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub uuid: Uuid,
}

// pub enum Roles {
//     User(String),
//     Courier(String),
//     Admin(String)
// }

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Courier {
    pub is_free: bool,
    pub rating: f64,
    pub uuid: Uuid,
}

#[derive(Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct Users {
    pub first_name: String,
    pub address: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub role: String,       // USER / COURIER / ADMIN
    pub is_blocked: bool,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

#[derive(Queryable, PartialEq, Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct CreateNewUser {
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub role: String,
}


#[derive(Queryable, PartialEq, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUserProfile {
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String
}

#[derive(Insertable, Debug)]
#[diesel(table_name = courier)]
#[derive(AsChangeset)]
pub struct UpdateCourierRating {
    pub rating: f64,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub message: String,
    pub status: bool,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message: String,
    pub status: bool,
}

