use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use env_logger;
use log::info;

mod config;
mod db;
mod models;
mod routes;

use crate::config::AppConfig;
use crate::db::connect_to_db;
use routes::{maps, reports, trends};

fn init_logging() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}

pub async fn run_server() -> std::io::Result<()> {
    init_logging();

    info!("Retrieving configuration...");
    let app_config = AppConfig::new().unwrap();
    info!("Configuration read successfuly");

    info!("Connecting to database...");
    let db_pool = connect_to_db(&app_config).await;
    info!("Connected to database successfuly");

    info!("Launching server in {}...", app_config.api.host);
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::new().finish()
            )
            .service(
                web::scope("/api/v2.0")
                    .service(web::scope("/trends").configure(trends::routes_config))
                    .service(web::scope("/maps").configure(maps::routes_config))
                    .service(web::scope("/reports").configure(reports::routes_config))
            )
    })
    .bind(app_config.api.host)?
    .run()
    .await
}
