use actix_web::{post, web, HttpResponse, Responder};
// use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

use crate::errors::server_error::ServerError;
use crate::schema::users;

use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2::ConnectionManager};
use diesel::{Insertable, Queryable};
use r2d2;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

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

/* ***************************************************************************
 *
 *                   Handler for Credentials struct
 *
 * **************************************************************************/

// impl Responder for NewUserCredentials {
//     type Error = actix_web::Error;
//     type Future = Ready<Result<HttpResponse, actix_web::Error>>;

//     fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
//         let res_body = serde_json::to_string(&self).unwrap();

//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(res_body)))
//     }
// }

#[post("/signup")]
async fn process_signup(
    creds: web::Form<NewUserCredentials>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServerError> {
    println!("{:?}", creds);
    let connection = pool.get()?;
    // if creds.password != creds.passconf {
    //     return "sad error";
    // }

    let new_user = NewUser::new(
        creds.email.clone(),
        creds.username.clone(),
        creds.password.clone(),
    );

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_results::<User>(&connection);

    // Ok(format!(
    //     "username: {}\npassword: {}\npassconf: {}",
    //     &creds.username, &creds.password, &creds.passconf
    // ))
    Ok(HttpResponse::Ok().body(format!("user registered {}", creds.username)))
}

// pub async fn process_signup(data: web::Form<NewUser>, pool: web::Data<Pool>) -> Result<HttpResponse, ServerError> {
//     use schema::users;

//     let connection = pool.get()?;

//     let new_user = NewUser::new(
//         data.username.clone(),
//         data.email.clone(),
//         data.password.clone());

//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .get_results::<User>(&connection)?;

//     println!("{:?}", data);
//     Ok(HttpResponse::Ok().body(format!("Successfully registered user: {}", data.username)))
// }
