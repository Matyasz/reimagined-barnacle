use actix_web::{post, web, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub passconf: String,
}

/* ***************************************************************************
 *
 *                   Handler for Credentials struct
 *
 * **************************************************************************/

impl Responder for Credentials {
    type Error = actix_web::Error;
    type Future = Ready<Result<HttpResponse, actix_web::Error>>;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
        let res_body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(res_body)))
    }
}

#[post("/signup")]
async fn process_signup(creds: web::Form<Credentials>) -> impl Responder {
    println!("{:?}", creds);

    format!(
        "username: {}\npassword: {}\npassconf: {}",
        &creds.username, &creds.password, &creds.passconf
    )
}
