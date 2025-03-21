use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Read, Write};

pub fn compress_file(input_file: &str, output_file: &str) {
    let mut input = File::open(input_file).expect("Unable to open input file");
    let mut data = Vec::new();
    input
        .read_to_end(&mut data)
        .expect("Unable to read input file contents");

    let output = File::create(output_file).expect("Unable to create output file");
    let mut encoder = GzEncoder::new(output, Compression::default());
    encoder.write_all(&data).expect("Unable to compress file");

    println!("File compressed successfully");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{self, Write};

    #[test]
    fn test_compress_file() -> io::Result<()> {
        let input_path = "test_input.txt";
        let output_path = "test_output.gz";

        // Create a test input file
        let mut file = File::create(input_path)?;
        writeln!(file, "This is a test file for compression.")?;

        // Compress the file
        compress_file(input_path, output_path);

        // Check if the compressed file exists and is non-empty
        let metadata = fs::metadata(output_path)?;
        assert!(metadata.len() > 0, "Compressed file should not be empty");

        // Cleanup
        fs::remove_file(input_path)?;
        fs::remove_file(output_path)?;

        Ok(())
    }
}
