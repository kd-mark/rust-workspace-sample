# File Upload & Compression API using Axum

This project provides a simple file upload and on-demand compression service using the Rust Axum framework. Uploaded files are stored in an `uploads` directory, and users can trigger file compression to store compressed versions in a `compressed` directory.

## Features
- Upload files via a multipart form.
- Store uploaded files in the `uploads` directory.
- Compress files on demand using gzip.
- Serve uploaded and compressed files statically.

## Installation
### Prerequisites
Ensure you have the following installed:
- Rust (latest stable version)
- Cargo
- Tokio (Async runtime)

### Clone the Repository
```sh
git clone <repo_url>
cd <repo_name>
```

### Install Dependencies
```sh
cargo build
```

## Running the Server
Start the server with:
```sh
cargo run
```
The server will be available at `http://localhost:3000`.

## API Endpoints

### 1. Upload File
**Endpoint:** `POST /upload-files`

**Description:** Upload a file using a multipart form.

**Example using curl:**
```sh
curl -X POST -F "file=@path/to/your/file" http://localhost:3000/upload-files
```

### 2. Compress Files On-Demand
**Endpoint:** `POST /compress-files`

**Description:** Compress all uploaded files that haven't been compressed yet.

**Example using curl:**
```sh
curl -X POST http://localhost:3000/compress-files
```

### 3. Serve Static Files
- Compressed files: `http://localhost:3000/compressed/<filename>.gz`

## Project Structure
```
├── src
│   ├── main.rs        # Main application logic
│   ├── upload.rs      # File upload logic
│   ├── compress.rs    # File compression logic
├── uploads/          # Directory for storing uploaded files
├── compressed/       # Directory for storing compressed files
└── Cargo.toml        # Rust dependencies and project configuration
```

## License
This project is licensed under the MIT License.

