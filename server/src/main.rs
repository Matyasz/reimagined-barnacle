#[macro_use]
extern crate diesel;

pub mod errors;
pub mod models;
pub mod schema;
pub mod user_actions;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

use user_actions::signup::signup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("oops database");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("oops pool");

    HttpServer::new(move || App::new().service(signup).data(pool.clone()))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
