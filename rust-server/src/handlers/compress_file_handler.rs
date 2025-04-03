use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use rust_server::compress_file;
use serde::Deserialize;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::services::{compressed_file_service::CompressedFileService, file_service::FileService};
use crate::{
    dtos::CreateCompressedFile,
    helpers::file::FileHelper,
    helpers::logger::{DefaultLogger, Logger},
    models::file::FileStatus,
};

// Define a struct to receive the compression level from the client
#[derive(Deserialize)]
pub struct CompressionLevel {
    level: u32,
}

pub struct CompressionHandler {
    logger: Arc<dyn Logger>,
    file_service: FileService,
    compressed_file_service: CompressedFileService,
}

impl CompressionHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            // Initialize the services
            file_service: FileService::new(pool.clone()),
            compressed_file_service: CompressedFileService::new(pool.clone()),

            // Initialize the logger
            logger: Arc::new(DefaultLogger::new::<CompressionHandler>()),
        }
    }
}

impl CompressionHandler {
    //// Endpoint to trigger file compression on demand
    pub async fn initiate(&self, id: String, query: CompressionLevel) -> impl IntoResponse {
        let id_uuid: Uuid = match id.parse() {
            Ok(uuid) => uuid,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid ID format: {}", id),
                );
            }
        };

        let file = match self.file_service.find_one(id_uuid).await {
            Ok(value) => value,
            Err(value) => return value,
        };

        let input_path = match FileHelper::get_uploaded_file_path(&file.file_ref) {
            Some(value) => value,
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    format!("File not found: {}", &file.file_ref),
                )
            }
        };

        let output_path = match FileHelper::get_compressed_file_path(&file.file_ref) {
            Some(value) => value,
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create output path: {}", &file.file_ref),
                )
            }
        };

        let (compression_level, compressed_file) = self
            .compressed_file_service
            .create(CreateCompressedFile {
                file_ref: file.file_ref,
                level: query.level,
                alg: "gzip".to_string(),
            })
            .await;

        match compressed_file {
            Ok(row) => {
                let logger = self.logger.clone();

                let id_uuid = Uuid::parse_str(&row.id).unwrap();
                let service = self.compressed_file_service.clone();

                tokio::task::spawn(async move {
                    match compress_file(&input_path, &output_path, compression_level).await {
                        Ok(_) => {
                            if let Err(e) = service.update_status(id_uuid, FileStatus::Passed).await
                            {
                                logger.error(&format!(
                                    "Failed to update file status to Passed: {}",
                                    e
                                ));
                            }
                        }
                        Err(_) => {
                            if let Err(db_err) =
                                service.update_status(id_uuid, FileStatus::Failed).await
                            {
                                logger.error(&format!(
                                    "Failed to update file status to Failed: {}",
                                    db_err
                                ));
                            }
                        }
                    }
                });

                (StatusCode::OK, serde_json::json!(row).to_string())
            }
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }

    pub async fn get_status(&self, id: String) -> impl IntoResponse {
        let id_uuid: Uuid = match id.parse() {
            Ok(uuid) => uuid,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid ID format: {}", id),
                );
            }
        };

        let file = match self.compressed_file_service.find_one(id_uuid).await {
            Ok(value) => value,
            Err(value) => return (StatusCode::INTERNAL_SERVER_ERROR, value.to_string()),
        };

        (StatusCode::OK, serde_json::json!(file).to_string())
    }
}
