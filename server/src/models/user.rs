use crate::schema::users;

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserCredentials {
    pub email: String,
    pub username: String,
    pub password: String,
    pub passconf: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl NewUser {
    pub fn new(email: String, username: String, password: String) -> Self {
        return NewUser {
            email: email,
            username: username,
            password: password,
        };
    }
}

