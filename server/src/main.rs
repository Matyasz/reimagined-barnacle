#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use actix_web::{App, HttpServer};

use models::credentials::process_signup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(process_signup))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
