use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};
use chrono::NaiveDateTime;
use crate::schema::schema::{users, courier};

extern crate uuid;
use uuid::Uuid;


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