use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};
use rust_server::compress_file;
use sqlx::types::Uuid;

use crate::app::AppState;
use crate::dtos::CompressionQuery;
use crate::helpers::env::Env;
use crate::services::{compressed_file_service::CompressedFileService, file_service::FileService};
use crate::{
    dtos::CreateCompressedFile,
    helpers::file::FileHelper,
    helpers::logger::{DefaultLogger, Logger},
    models::file::FileStatus,
};

pub struct CompressionHandler {
    env: Arc<Env>,
    logger: Arc<dyn Logger>,
    file_service: FileService,
    compressed_file_service: CompressedFileService,
}

impl CompressionHandler {
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            // Initialize the services
            env: state.env.clone(),
            file_service: FileService::new(state.pool.clone()),
            compressed_file_service: CompressedFileService::new(state.pool.clone()),

            // Initialize the logger
            logger: Arc::new(DefaultLogger::new::<CompressionHandler>()),
        }
    }
}

impl CompressionHandler {
    //// Endpoint to trigger file compression on demand
    pub async fn initiate(&self, id: String, query: CompressionQuery) -> impl IntoResponse {
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
            Err(e) => return (StatusCode::NOT_FOUND, format!("File record not found: {e}")),
        };

        let input_path =
            match FileHelper::get_uploaded_file_path(&file.file_ref, &self.env.uploads_dir) {
                Some(value) => value,
                None => {
                    return (
                        StatusCode::NOT_FOUND,
                        format!("File not found: {}", &file.file_ref),
                    )
                }
            };

        let output_path =
            match FileHelper::get_compressed_file_path(&file.file_ref, &self.env.compressed_dir) {
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
                    logger.debug(&format!("Starting compression task(id: {})...", id_uuid));
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
                    logger.debug(&format!("Compression task(id: {}) completed.", id_uuid));
                });

                (StatusCode::OK, serde_json::json!(row).to_string())
            }
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create compressed file: {e}")),
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
