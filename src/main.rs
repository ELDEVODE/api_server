use actix_web::{web, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get port from environment variable or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    println!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            // Add your routes and middleware here
            .route("/", web::get().to(|| async { "Hello, World!" }))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}