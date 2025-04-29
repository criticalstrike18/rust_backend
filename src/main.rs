mod auth;
mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;

use actix_web::{App, HttpServer, middleware, web};
use config::AppConfig;
use db::create_pool_with_retry;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Load configuration
    let config = Arc::new(AppConfig::from_env().expect("Failed to load configuration"));
    log::info!("Environment: {}", config.service.environment);

    // Database connection with retry
    let database_url = &config.database.url;
    log::info!("Connecting to database at '{}'", database_url);

    let pool = create_pool_with_retry(database_url, config.database.max_connections, 5)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    log::info!("Running database migrations...");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => log::info!("Database migrations completed successfully"),
        Err(e) => {
            log::error!("Failed to run migrations: {:?}", e);
            log::warn!("Continuing without migrations - tables should already exist");
        }
    }

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
            // Sync routes
            .configure(routes::sync::config)
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