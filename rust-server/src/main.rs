mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::{compress_file, upload_file};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Create a new Axum router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/upload-files", post(upload_file::upload_files))
        .route("/compress-files", post(compress_file::compress_all_files));

    // Define the address for the server to listen on
    let listener = match TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind TcpListener to server {e}");
            return;
        }
    };
    // Start the server
    axum::serve(listener, app).await.unwrap();
}
