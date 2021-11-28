mod models;
mod config;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::models::Status;
use std::io;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    web::HttpResponse::Ok().json(Status{status: "ZHOPKA".to_string()})
    // HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}