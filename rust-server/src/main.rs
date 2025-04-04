mod app;
mod database;
mod dtos;
mod handlers;
mod helpers;
mod middlewares;
mod models;
mod services;

use std::sync::Arc;

use app::App;

use database::sqlx::SqlxPgPool;
use helpers::{
    env::Env,
    logger::{DefaultLogger, Logger},
};

#[tokio::main]
async fn main() {
    let logger = DefaultLogger::new::<App>();
    let env = Arc::new(Env::load(&logger));

    let pool = SqlxPgPool::new(&logger).connect(&env.database_url).await;

    if let Err(e) = pool {
        logger.error(&format!("Could initialized database connection: {e}"));
        return;
    }

    let app = App::new(Arc::new(pool.unwrap()), env.clone());

    // Define the address for the server to listen on
    let ip_addr = format!("{}:{}", env.host, env.port);
    app.run_server(&ip_addr, logger).await;
}
