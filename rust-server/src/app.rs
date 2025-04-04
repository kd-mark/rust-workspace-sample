use super::handlers::{
    compress_file_handler::CompressionHandler, upload_file_handler::UploadFileHandler,
};
use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::{
    dtos::CompressionQuery,
    helpers::{env::Env, logger::Logger},
    middlewares::{auth_guard, log_requests},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Arc<Env>,
    pub pool: Arc<PgPool>,
}

// Renamed App struct to AppState for clarity
#[derive(Debug, Clone)]
pub struct App {
    state: Arc<AppState>,
}

impl App {
    pub fn new(pool: Arc<PgPool>, env: Arc<Env>) -> Self {
        Self {
            state: Arc::new(AppState { pool, env }),
        }
    }
}

impl App {
    fn route(app_state: Arc<AppState>) -> Router {
        Router::new()
            .nest("/files", Self::upload_handler_routes())
            .nest("/compressed-files", Self::compression_handler_routes())
            .layer(middleware::from_fn(log_requests::log_requests))
            .with_state(app_state)
    }

    // Renamed bootstrap function to run_server for clarity
    pub async fn run_server(self, ip_addr: &str, logger: impl Logger) {
        // Create a new Axum router
        let app = Self::route(self.state.clone());
        // Serve the files from the uploads directory
        let app = app.nest_service(
            self.state.env.uploads_dir.as_str(),
            ServeDir::new(self.state.env.uploads_dir.as_str()),
        );

        let listener = match TcpListener::bind(ip_addr).await {
            Ok(listener) => listener,
            Err(e) => {
                logger.error(&format!("Failed to bind TcpListener to server {e}"));
                return;
            }
        };

        logger.log(&format!("Application is running http://{ip_addr}"));

        // Start the server
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("{e}")
        }
    }
}

impl App {
    fn upload_handler_routes() -> Router<Arc<AppState>> {
        Router::new()
            .route(
                "/upload",
                post(
                    |State(state): State<Arc<AppState>>, multipart: Multipart| async move {
                        let upload_file_handler = UploadFileHandler::new(state);
                        upload_file_handler.upload_files(multipart).await
                    },
                ),
            )
            .route(
                "/{id}",
                get(
                    |State(state): State<Arc<AppState>>, Path(id): Path<String>| async move {
                        let upload_file_handler = UploadFileHandler::new(state);
                        upload_file_handler.get_file(id).await
                    },
                ),
            )
            .fallback(|| async { (StatusCode::NOT_FOUND, "Route not found".to_string()) })
    }
}

impl App {
    fn compression_handler_routes() -> Router<Arc<AppState>> {
        Router::new()
            .route(
                "/{file_id}/compress",
                post(
                    |State(state): State<Arc<AppState>>,
                     Path(file_id): Path<String>,
                     Query(query): Query<CompressionQuery>| async move {
                        let compression_handler = Arc::new(CompressionHandler::new(state.clone()));
                        compression_handler.initiate(file_id, query).await
                    },
                ),
            )
            .route(
                "/{id}/status",
                get(|State(state): State<Arc<AppState>>, Path(id)| async move {
                    let compression_handler = Arc::new(CompressionHandler::new(state.clone()));
                    compression_handler.get_status(id).await
                }),
            )
            .layer(middleware::from_fn(auth_guard::auth_guard))
    }
}
