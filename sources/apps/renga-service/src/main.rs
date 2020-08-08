use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::env;

#[derive(Deserialize)]
struct Greeting {
    name: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!!")
}

async fn time() -> impl Responder {
    let current: DateTime<Utc> = Utc::now();
    HttpResponse::Ok().body(format!("current: {}", current.to_rfc3339()))
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

/*
async fn hello(greeting: web::Path<Greeting>) -> impl Responder {
    format!("Hello, {}!", greeting.name)
}
*/

async fn hello(greeting: web::Path<Greeting>) -> actix_web::Result<String> {
    Ok(format!("Hello, {}!", greeting.name))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("running...");

    let port = env::var("RENGA_PORT").unwrap_or("8088".to_string());
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/time", web::get().to(time))
            .service(index3)
            .route("/hello/{name}", web::get().to(hello))
    })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}
