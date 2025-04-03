use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    dtos::CreateFile,
    helpers::{env::Env, logger::DefaultLogger},
    services::file_service::FileService,
};

pub struct UploadFileHandler {
    file_service: FileService,
}

impl UploadFileHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            file_service: FileService::new(pool),
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
                    if let Err(e) = Self::save_file(file_name.to_owned(), &data).await {
                        return (
                            StatusCode::BAD_REQUEST,
                            format!("Failed to save file: {}", e),
                        );
                    }

                    let file_service = self.file_service.clone();
                    let task = tokio::spawn(async move {
                        let Env { host, port, .. } =
                            Env::load(&DefaultLogger::new::<UploadFileHandler>());

                        file_service
                            .create(CreateFile {
                                file_ref: format!("http://{host}:{port}/uploads/{file_name}"),
                                size: data.len() as u64,
                            })
                            .await
                    });

                    uploaded_file_tasks.push(task);
                }
            }
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
    /// Saves the uploaded file to the `uploads` directory
    pub async fn save_file(file_name: String, data: &[u8]) -> io::Result<()> {
        let save_path = PathBuf::from("uploads").join(file_name);
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
