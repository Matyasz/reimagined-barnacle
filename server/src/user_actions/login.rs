#[path = "../schema.rs"]
pub mod schema;

use actix_session::Session;
use actix_web::{post, web, Responder};

use crate::errors::server_error::ServerError;
use crate::models::user::{LoginCredentials, User};

use uuid::Uuid;

use argonautica::Verifier;
use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/login")]
async fn login(
    credentials: String,
    pool: web::Data<Pool>,
    session: Session,
) -> Result<impl Responder, ServerError> {
    use schema::users::dsl::{email, users};

    let creds: LoginCredentials = serde_json::from_str(&credentials).unwrap();
    let secret = std::env::var("SECRET_KEY")?;

    let connection = pool.get()?;
    let user = users
        .filter(email.eq(&creds.email))
        .first::<User>(&connection)?;

    let valid_login = Verifier::default()
        .with_hash(user.password)
        .with_password(creds.password)
        .with_secret_key(secret)
        .verify()?;

    if valid_login {
        let session_uuid = Uuid::new_v4();
        let _r = session.insert("session_id", session_uuid);

        println!("{:?}", session.entries());

        Ok(format!("{}", session_uuid))
    } else {
        Ok(String::from("Incorrect Password"))
    }
}
