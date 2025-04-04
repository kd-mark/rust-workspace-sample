use crate::{dtos::CreateFile, models::file::File};
use axum::http::StatusCode;
use sqlx::{postgres::PgRow, types::Uuid, PgPool};
use sqlx::Row;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FileService {
    pool: Arc<PgPool>,
}

impl FileService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl FileService {
    pub async fn find_one(&self, id: Uuid) -> Result<File, (StatusCode, String)> {
        let file = sqlx::query_as!(
            File,
            "SELECT id, size, file_ref FROM files WHERE id = $1",
            id
        )
        .fetch_one(&*self.pool)
        .await;
        let file = match file {
            Ok(file) => file,
            Err(_) => {
                return Err((
                    StatusCode::NOT_FOUND,
                    format!("File record not found: {}", id),
                ));
            }
        };
        Ok(file)
    }

    pub async fn create(&self, file: CreateFile) -> Result<File, (StatusCode, String)> {
        let file = sqlx::query("INSERT INTO files (size, file_ref) VALUES ($1, $2) RETURNING id, size, file_ref")
            .bind(file.size as i64)
            .bind(file.file_ref)
            .fetch_one(&*self.pool)
            .await.map(|row: PgRow| File {
                id: row.get("id"),
                size: row.get("size"),
                file_ref: row.get("file_ref"),
            });
       
        let file = match file {
            Ok(file) => file,
            Err(_) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create file".to_string()));
            }
        };
        
        Ok(file)
    }
}
