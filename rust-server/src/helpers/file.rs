use std::path::PathBuf;

pub struct FileHelper;

impl FileHelper {
    pub fn get_compressed_file_path(file_ref: &str) -> Option<String> {
        let compressed_dir = PathBuf::from("compressed");
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
    
    pub fn get_uploaded_file_path(file_ref: &str) -> Option<String> {
        let input_path_buf = PathBuf::from("uploads").join(file_ref);
        let input_path = match input_path_buf.to_str() {
            Some(path) => path.to_string(),
            None => {
                return None;
            }
        };
    
        Some(input_path)
    }
}