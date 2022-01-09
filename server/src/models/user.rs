use crate::schema::users;

use argonautica::Hasher;
use diesel::{Insertable, Queryable};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserCredentials {
    pub email: String,
    pub name: String,
    pub password: String,
    pub passconf: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String,
}

impl NewUser {
    pub fn new(email: String, name: String, password: String) -> Self {
        dotenv().ok();

        let secret =
            std::env::var("SECRET_KEY").expect("Environment variable SECRET_KEY must be set.");

        let hashed_pass = Hasher::default()
            .with_password(password)
            .with_secret_key(secret)
            .hash()
            .unwrap();

        return NewUser {
            email: email,
            name: name,
            password: hashed_pass,
        };
    }
}
