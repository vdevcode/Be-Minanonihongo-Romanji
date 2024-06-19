use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod config;
mod handlers;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .configure(routes::init)
    })
    .bind(address)?
    .run()
    .await
}
