// src/main.rs
mod auth;
mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;

use actix_web::{App, HttpServer, middleware, web};
use config::AppConfig;
use db::create_pool;
use sqlx::{PgPool, Row};
use dotenv::dotenv;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Load configuration
    let config = Arc::new(AppConfig::from_env().expect("Failed to load configuration"));
    log::info!("Environment: {}", config.service.environment);

    // Database connection
    let database_url = &config.database.url;
    log::info!("Connecting to database at '{}'", database_url);

    let pool = create_pool(database_url, config.database.max_connections)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    log::info!("Running database migrations...");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => log::info!("Database migrations completed successfully"),
        Err(e) => {
            log::error!("Failed to run migrations: {:?}", e);

            // Emergency fallback: try to create tables directly if migrations fail
            if let Err(fallback_err) = ensure_tables_exist(&pool).await {
                log::error!("Emergency table creation also failed: {:?}", fallback_err);
                panic!(
                    "Cannot initialize database. Please check your database connection and schema."
                );
            } else {
                log::warn!("Used emergency table creation as fallback");
            }
        }
    }

    async fn ensure_tables_exist(pool: &PgPool) -> Result<(), sqlx::Error> {
        // Check if podcast tables exist
        let table_check =
            sqlx::query("SELECT to_regclass('podcast_channels') IS NOT NULL as exists")
                .fetch_one(pool)
                .await?;

        let tables_exist: bool = table_check.get("exists");

        if !tables_exist {
            log::info!("Podcast tables don't exist. Creating them manually...");

            // Get the SQL content from the migration file
            let sql = include_str!("../migrations/20250430000000_create_podcast_tables.sql");

            // Execute the SQL
            sqlx::query(sql).execute(pool).await?;
            log::info!("Manually created podcast tables");
        }

        Ok(())
    }

    // Start synchronization service
    services::start_sync_service(
        pool.clone(),
        config.sessionize.interval,
        config.sessionize.url.clone(),
    );

    // Start server
    let server_config = config.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            // Middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "1.0.0")))
            // Routes
            .service(routes::users::sign)
            .service(routes::conference::get_conference)
            // Votes routes
            .service(routes::votes::get_votes)
            .service(routes::votes::post_vote)
            .service(routes::votes::get_all_votes)
            // Feedback routes
            .service(routes::feedback::post_feedback)
            .service(routes::feedback::get_feedback_summary)
            // Admin routes
            .service(routes::admin::sessionize_sync)
            .service(routes::admin::get_time)
            .service(routes::admin::set_time)
            .service(routes::admin::add_admin_session)
            .service(routes::admin::add_admin_speaker)
            .service(routes::admin::add_admin_room)
            .service(routes::admin::add_admin_category)
            // Session management routes
            .service(routes::sessions::get_sessions)
            .service(routes::sessions::get_categories)
            .service(routes::sessions::get_rooms)
            .service(routes::sessions::get_speakers)
            .service(routes::sessions::get_session_speakers)
            .service(routes::sessions::get_session_categories)
            .service(routes::sessions::send_session)
            .service(routes::sessions::send_room)
            .service(routes::sessions::send_session_speaker)
            .service(routes::sessions::send_session_categories)
            // Podcast routes
            .service(routes::podcast::send_podcast_request)
            .service(routes::podcast::import_podcast)
            .service(routes::podcast::get_all_podcasts)
            // Health check
            .route("/healthz", web::get().to(|| async { "OK" }))
    })
    .bind(format!(
        "{}:{}",
        server_config.server.host, server_config.server.port
    ))?
    .run()
    .await
}
