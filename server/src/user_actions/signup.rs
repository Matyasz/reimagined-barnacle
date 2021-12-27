use actix_web::{post, web, HttpResponse};
// use futures::future::{ready, Ready};

use crate::errors::server_error::ServerError;
use crate::models::user::{NewUser, NewUserCredentials, User};
use crate::schema::users;

use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/signup")]
async fn signup(
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

    let _db_res = diesel::insert_into(users::table)
        .values(&new_user)
        .get_results::<User>(&connection);

    Ok(HttpResponse::Ok().body(format!("user registered {}", creds.username)))
}

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
