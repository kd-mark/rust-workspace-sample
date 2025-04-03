use super::handlers::{
    compress_file_handler::CompressionHandler, upload_file_handler::UploadFileHandler,
};
use axum::{
    extract::{Multipart, Path, Query, State},
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::{helpers::logger::Logger, middlewares::auth_guard};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct AppState {
    pool: PgPool,
}

// Renamed App struct to AppState for clarity
#[derive(Debug, Clone)]
pub struct App {
    state: Arc<AppState>,
}

impl App {
    pub fn new(pool: PgPool) -> Self {
        Self {
            state: Arc::new(AppState { pool }),
        }
    }
}

impl App {
    fn route(app_state: Arc<AppState>) -> Router {
        Router::new()
            .route(
                "/upload",
                post(
                    |State(state): State<Arc<AppState>>, multipart: Multipart| async move {
                        let upload_file_handler =
                            UploadFileHandler::new(Arc::new(state.pool.clone()));
                        upload_file_handler.upload_files(multipart).await
                    },
                ),
            )
            .route(
                "/:id/initiate",
                post(
                    |State(state): State<Arc<AppState>>, Path(path), Query(query)| async move {
                        let compression_handler =
                            Arc::new(CompressionHandler::new(Arc::new(state.pool.clone())));
                        compression_handler.initiate(path, query).await
                    },
                ),
            )
            .route(
                "/:id/status",
                get(
                    |State(state): State<Arc<AppState>>, Path(path)| async move {
                        let compression_handler =
                            Arc::new(CompressionHandler::new(Arc::new(state.pool.clone())));
                        compression_handler.get_status(path).await
                    },
                ),
            )
            .layer(middleware::from_fn(auth_guard::auth_guard))
            .with_state(app_state)
    }

    // Renamed bootstrap function to run_server for clarity
    pub async fn run_server(self, ip_addr: &str, logger: impl Logger) {
        // Create a new Axum router
        let app = Self::route(self.state);
        // Serve the files from the uploads directory
        let app = app.nest_service("/uploads", ServeDir::new("uploads"));

        let listener = match TcpListener::bind(ip_addr).await {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind TcpListener to server {e}");
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
