mod config;
mod db;
mod models;
mod routes;

use dotenv::dotenv;
use actix_web::{get, App, HttpResponse, HttpServer, Responder,web};
use config::AppConfig;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Load configuration from environment
    let config = AppConfig::from_env().expect("Failed to load configuration");
    let server_config = config.server.clone();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| config.database.url.clone());

    if !sqlx::Postgres::database_exists(&database_url).await.unwrap_or(false) {
        println!("Creating database...");
        sqlx::Postgres::create_database(&database_url).await
            .expect("Failed to create database");
    }
    
    // Connect to the database
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");
    
    // Run migrations
    println!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

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
