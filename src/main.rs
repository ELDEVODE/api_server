use actix_web::{web, App, HttpServer};
use std::env;

#[get("/")]
async fn greet() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(|| App::new().route("/", web::get().to(|| async { "Hello, world!" })))
        .bind(("0.0.0.0", port.parse().unwrap()))?
        .run()
        .await
}