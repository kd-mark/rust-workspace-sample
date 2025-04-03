use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub size: i64,
    pub file_ref: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "status_enum", rename_all = "lowercase")]
pub enum FileStatus {
    Compressing,
    Passed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CompressedFile {
    pub id: String,
    pub status: FileStatus,
    pub file_ref: String,
    pub level: i32,
    pub alg: String,
}
