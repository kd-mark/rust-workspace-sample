use reqwest::multipart;
use reqwest::Client;
use std::{
    env,
    fs::File,
    io::{self, Read},
    path::Path,
};

async fn read_file(file_path: &str) -> io::Result<(String, Vec<u8>)> {
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(String::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    let mut buffer = Vec::new();
    File::open(file_path)?.read_to_end(&mut buffer)?;

    Ok((file_name, buffer))
}

async fn upload_file(
    client: &Client,
    url: &str,
    file_name: String,
    file_data: Vec<u8>,
) -> Result<String, reqwest::Error> {
    let part = multipart::Part::bytes(file_data)
        .file_name(file_name.clone())
        .mime_str("application/octet-stream")
        .unwrap_or_else(|_| panic!("Failed to set MIME type")); // Should not fail in practice

    let form = multipart::Form::new().part("file", part);

    let response = client.post(url).multipart(form).send().await?;

    if response.status().is_success() {
        println!("File '{}' uploaded successfully", file_name);
        let reponse_body = response.text().await?;
        Ok(reponse_body)
    } else {
        panic!("Failed to upload file: {:?}", response.status());
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    match read_file(file_path).await {
        Ok((file_name, file_data)) => {
            let client = Client::new();
            let url = "http://localhost:3000/uploader/upload";
            match upload_file(&client, url, file_name, file_data).await {
                Ok(data) => println!("{data}"),
                Err(err) => {
                    eprintln!("Error uploading file: {}", err);
                }
            }
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
