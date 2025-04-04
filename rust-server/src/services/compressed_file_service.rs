use std::sync::Arc;

use flate2::Compression;
use sqlx::postgres::PgQueryResult;
use sqlx::Row;
use sqlx::{postgres::PgRow, types::Uuid, PgPool};

use crate::dtos::CreateCompressedFile;
use crate::models::file::CompressedFile;
use crate::models::file::FileStatus;

#[derive(Debug, Clone)]
pub struct CompressedFileService {
    pool: Arc<PgPool>,
}

impl CompressedFileService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl CompressedFileService {
    pub async fn create(
        &self,
        create_compressed_file: CreateCompressedFile,
    ) -> (Compression, Result<CompressedFile, sqlx::Error>) {
        let compression_level = match create_compressed_file.level {
            0 => Compression::none(),
            1..=9 => Compression::new(create_compressed_file.level),
            _ => Compression::default(),
        };

        let compressed_file = sqlx::query("INSERT INTO compressed_files (status, file_ref, level, alg) VALUES ($1, $2, $3, $4) RETURNING id::text, status, file_ref, level, alg")
            .bind(FileStatus::Compressing)
            .bind(create_compressed_file.file_ref)
            .bind(compression_level.level() as i32)
            .bind(create_compressed_file.alg)
            .fetch_one(&*self.pool)
            .await.map(|row: PgRow| CompressedFile {
                id: row.get("id"),
                status: row.get("status"),
                file_ref: row.get("file_ref"),
                level: row.get("level"),
                alg: row.get("alg"),
            });
        (compression_level, compressed_file)
    }

    pub async fn update_status(
        &self,
        id: Uuid,
        status: FileStatus,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE compressed_files SET status = $1 WHERE id = $2")
            .bind(status)
            .bind(id)
            .execute(&*self.pool)
            .await
    }

    pub async fn find_one(&self, id: Uuid) -> Result<CompressedFile, sqlx::Error> {
        sqlx::query("SELECT id::text, status, file_ref, level, alg FROM compressed_files WHERE id = $1")
            .bind(id)
            .fetch_one(&*self.pool)
            .await
            .map(|row: PgRow| CompressedFile {
                id: row.get("id"),
                status: row.get("status"),
                file_ref: row.get("file_ref"),
                level: row.get("level"),
                alg: row.get("alg"),
            })
    }
}
