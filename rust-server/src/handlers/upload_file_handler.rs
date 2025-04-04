use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};
use sqlx::types::Uuid;
use std::{
    fs::File, io::{self, Write}, path::PathBuf, sync::Arc, time::{SystemTime, UNIX_EPOCH}
};

use crate::{
    app::AppState, dtos::CreateFile, helpers::{env::Env, logger::{DefaultLogger, Logger}}, services::file_service::FileService
};

pub struct UploadFileHandler {
    env: Arc<Env>,
    file_service: FileService,
    logger: Arc<dyn Logger>,
}

impl UploadFileHandler {
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            env: state.env.clone(),
            file_service: FileService::new(state.pool.clone()),

            // Initialize the logger
            logger: Arc::new(DefaultLogger::new::<UploadFileHandler>()),
        }
    }
}

impl UploadFileHandler {
    /// Handles file uploads from a multipart form request
    pub async fn upload_files(&self, mut multipart: Multipart) -> impl IntoResponse {
        let mut uploaded_file_tasks = vec![];
        while let Ok(Some(field)) = multipart.next_field().await {
            let file_name = match Self::extract_filename(&field) {
                Ok(value) => value,
                Err(value) => return (StatusCode::BAD_REQUEST, value),
            };

            match field.bytes().await {
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        "Failed to read file data!".to_string(),
                    )
                }
                Ok(data) => {
                    let env = self.env.clone();
                    let file_service = self.file_service.clone();
                    let logger = self.logger.clone();
                    let task = tokio::spawn(async move {
                        if let Err(e) = Self::save_file(file_name.to_owned(), &data, &env.uploads_dir).await {
                            logger.info(&format!("Failed to save file: {}", e));
                            return Err((
                                StatusCode::BAD_REQUEST,
                                format!("Failed to save file: {}", e),
                            ));
                        }

                        file_service
                            .create(CreateFile {
                                file_ref: format!("/uploads/{file_name}"),
                                size: data.len() as u64,
                            })
                            .await
                    });
                    uploaded_file_tasks.push(task);
                }
            };
        }

        let files = futures::future::join_all(uploaded_file_tasks)
            .await
            .into_iter()
            .filter_map(|file| match file {
                Ok(file) => match file {
                    Ok(file) => Some(file),
                    Err(_) => None,
                },
                Err(_) => None,
            })
            .collect::<Vec<_>>();

        (StatusCode::CREATED, serde_json::to_string(&files).unwrap())
    }
}

impl UploadFileHandler {
    pub async fn get_file(&self, id: String) -> impl IntoResponse {
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
            Ok(file) => file,
            Err(value) => return value,
        };

        (StatusCode::OK, serde_json::to_string(&file).unwrap())
    }
}

impl UploadFileHandler {
    /// Saves the uploaded file to the `uploads` directory
    pub async fn save_file(file_name: String, data: &[u8], uploads_dir: &str) -> io::Result<()> {
        let save_path = PathBuf::from(uploads_dir).join(file_name);
        let mut file = File::create(save_path)?; // Create a new file in the uploads directory
        file.write_all(data) // Write the received data into the file
    }

    fn extract_filename(field: &axum::extract::multipart::Field<'_>) -> Result<String, String> {
        let mut file_name: String = String::from(field.file_name().unwrap_or("unnamed"))
            .split(&[' ', '-', ':', '\''])
            .collect();

        let now = SystemTime::now();

        match now.duration_since(UNIX_EPOCH) {
            Ok(duration) => file_name.insert_str(0, &format!("{}_", duration.as_secs())),
            Err(_) => return Err(format!("Time went backward!")),
        };

        Ok(file_name)
    }
}
