use std::path::PathBuf;

pub struct FileHelper;

impl FileHelper {
    pub fn get_compressed_file_path(file_ref: &str, compressed_dir: &str) -> Option<String> {
        let compressed_dir = PathBuf::from(compressed_dir);
        if let Err(_) = std::fs::create_dir_all(&compressed_dir) {
            return None;
        }
    
        let output_path = compressed_dir.join(file_ref).with_extension("gz");
        let output_path = match output_path.to_str() {
            Some(path) => path.to_string(),
            None => {
                return None;
            }
        };
    
        Some(output_path)
    }
    
    pub fn get_uploaded_file_path(file_ref: &str, uploads_dir: &str) -> Option<String> {
        let uploads_dir = PathBuf::from(uploads_dir);
        if let Err(_) = std::fs::create_dir_all(&uploads_dir) {
            return None;
        }

        let input_path_buf = PathBuf::from(uploads_dir).join(file_ref);
        let input_path = match input_path_buf.to_str() {
            Some(path) => path.to_string(),
            None => {
                return None;
            }
        };
    
        Some(input_path)
    }
}