use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct Greeting {
    name: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!!!")
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

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
            .route("/hello/{name}", web::get().to(hello))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
