use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use chrono::NaiveDateTime;
use crate::schema::schema::users;

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

#[derive(Queryable, PartialEq, Debug, Serialize)]
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

#[derive(Queryable, PartialEq, Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct CreateNewUser {
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct RetrieveUserResponse {
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
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

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
           .content_type(ContentType::json())
           .body(res_body)
    }
}
