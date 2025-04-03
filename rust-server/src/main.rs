mod app;
mod handlers;
mod database;
mod helpers;
mod middlewares;
mod models;
mod services;
mod dtos;

use std::sync::Arc;

use app::App;
use axum::{middleware, routing::get, Router};

use database::sqlx::SqlxPgPool;
use helpers::{
    env::Env,
    logger::{DefaultLogger, Logger},
};
use middlewares::log_requests::log_requests;
use sqlx::PgPool;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let logger = DefaultLogger::new::<App>();
    let Env { database_url, host, port } = Env::load(&logger);

    let pool = SqlxPgPool::new(&logger).connect(&database_url).await;

    if let Err(e) = pool {
        logger.error(&format!("Could initialized database connection: {e}"));
        return;
    }

    let app = App::new(pool.unwrap());

    // Define the address for the server to listen on
    let ip_addr = format!("{}:{}", host, port);
    app.run_server(&ip_addr, logger).await;
}
