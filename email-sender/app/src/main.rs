#![allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use common::prelude::*;
use google_oauth;

mod mailer;

#[actix_web::main]
async fn main() -> Result<(), DynError> {
    dotenv::dotenv()?;

    let auth = google_oauth::get_auth().await?;
    let email = mailer::generate_email_sender(&auth).await?;
    email.send_all_email_smtp().await?;

    // HttpServer::new(|| {
    //     App::new()
    //         .service(hello)
    //         .service(echo)
    //         .route("/hey", web::get().to(manual_hello))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await?;

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
