use crate::{dtos::CreateFile, models::file::File};
use sqlx::Row;
use sqlx::{postgres::PgRow, types::Uuid, PgPool};
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
    pub async fn find_one(&self, id: Uuid) -> Result<File, sqlx::Error> {
        sqlx::query(
            "SELECT id::text, size, file_ref FROM files WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
        .map(|row: PgRow| File {
            id: row.get("id"),
            file_ref: row.get("file_ref"),
            size: row.get("size"),
        })
    }

    pub async fn create(&self, file: CreateFile) -> Result<File, sqlx::Error> {
        sqlx::query(
            "INSERT INTO files (size, file_ref) VALUES ($1, $2) RETURNING id::text, size, file_ref",
        )
        .bind(file.size as i64)
        .bind(file.file_ref)
        .fetch_one(&*self.pool)
        .await
        .map(|row: PgRow| File {
            id: row.get("id"),
            size: row.get("size"),
            file_ref: row.get("file_ref"),
        })
    }
}
