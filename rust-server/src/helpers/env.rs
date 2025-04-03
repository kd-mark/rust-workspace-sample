use std::env;

use super::logger::Logger;

pub struct Env {
    pub database_url: String,
    pub host: String,
    pub port: String,
}

impl Env {
    pub fn load(logger: &impl Logger) -> Self {
        logger.log("Loading environmental viarables");
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL");
        if let Err(_) = &database_url {
            logger.warn("Missing environment variable: DATABASE_URL");
        };

        let host = env::var("HOST");
        if let Err(_) = &host {
            logger.warn("Missing environment variable: HOST");
        };

        let port = env::var("PORT");
        if let Err(_) = &port {
            logger.warn("Missing environment variable: PORT");
        };

        Env {
            database_url: database_url.unwrap_or("".to_owned()),
            host: host.unwrap_or("".to_owned()),
            port: port.unwrap_or("".to_owned()),
        }
    }
}
