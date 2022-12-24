use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schemas::schema::*;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub is_blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Address {
    pub id: i32,
    pub user_id: i32,
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Role {
    Undefined,
    User,
    Admin
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: i32,
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub is_blocked: bool,
}

#[derive(Insertable, Debug)]
#[diesel(belongs_to(Users))]
#[diesel(table_name = addresses)]
pub struct UserAddresses {
    pub id: i32,
    pub user_id: i32,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub id: i32,
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct CreateNewUser {
    pub id: i32,
    pub first_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String
}
