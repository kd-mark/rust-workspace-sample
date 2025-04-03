use serde::Deserialize;

// Define a struct to receive the compression level from the client
#[derive(Deserialize)]
pub struct CompressionLevel {
    pub level: u32,
}

#[derive(Deserialize)]
pub struct CreateCompressedFile {
    pub file_ref: String,
    pub level: u32,
    pub alg: String,
}

#[derive(Deserialize)]
pub struct CreateFile {
    pub file_ref: String,
    pub size: u64,
}
