mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::{compress_file, upload_file};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let compressed = Router::new()
        .route("/compress", post(compress_file::compress_all_files))
        .nest_service("/files", ServeDir::new("compressed"));

    let uploads = Router::new()
        .route("/upload", post(upload_file::upload_files))
        .nest_service("/files", ServeDir::new("uploads"));

    // Create a new Axum router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/uploader", uploads)
        .nest("/compressor", compressed)
        .fallback(|| async { r#"{"status":404,"message":"Resource Not Found"}"# });

    // Define the address for the server to listen on
    let listener = match TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind TcpListener to server {e}");
            return;
        }
    };

    // Start the server
    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}")
        }
    };
}
