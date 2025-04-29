mod config;
mod db;
mod models;
mod routes;

use actix_web::{get, App, HttpResponse, HttpServer, Responder,web};
use config::AppConfig;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration from environment
    let config = AppConfig::from_env().expect("Failed to load configuration");
    let server_config = config.server.clone();

    let pool = db::create_pool(
        &config.database.url, 
        config.database.max_connections
    )
    .await
    .expect("Failed to create database pool");

    HttpServer::new(move|| {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(routes::users::sign)
        .service(hello)
    })
    .bind((server_config.host, server_config.port))?
    .run()
    .await
}
