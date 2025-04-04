use std::env;

use super::logger::Logger;

#[derive(Debug, Clone)]
pub struct Env {
    pub database_url: String,
    pub host: String,
    pub port: String,
    pub uploads_dir: String,
    pub compressed_dir: String,
}

impl Env {
    pub fn load(logger: &dyn Logger) -> Self {
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

        let uploads_dir = env::var("UPLOADS_DIR");
        if let Err(_) = &uploads_dir {
            logger.warn("Missing environment variable: UPLOADS_DIR");
        };

        let compressed_dir = env::var("COMPRESSED_DIR");
        if let Err(_) = &compressed_dir {
            logger.warn("Missing environment variable: COMPRESSED_DIR");
        };

        Env {
            database_url: database_url.unwrap_or("".to_owned()),
            host: host.unwrap_or("".to_owned()),
            port: port.unwrap_or("".to_owned()),
            uploads_dir: uploads_dir.unwrap_or("".to_owned()),
            compressed_dir: compressed_dir.unwrap_or("".to_owned()),
        }
    }
}
