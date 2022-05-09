#[macro_use]
extern crate diesel;

pub mod errors;
pub mod models;
pub mod schema;
pub mod user_actions;

use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::web::Data;
use actix_web::{cookie::Key, http, App, HttpServer};

use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

use user_actions::login::login;
use user_actions::signup::signup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("oops database");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("oops pool");

    let store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(36000);

        let app_pool = Data::new(pool.clone());

        // Repeat the key 3 times because the Redis key needs to be 64 bytes...
        let secret = std::env::var("SECRET_KEY").expect("").repeat(3);
        let secret_key = Key::from(secret.as_bytes());

        App::new()
            .wrap(cors)
            .wrap(SessionMiddleware::new(store.clone(), secret_key))
            .service(signup)
            .service(login)
            .app_data(app_pool)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
