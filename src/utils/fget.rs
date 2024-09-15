use std::fs::File;
use std::io::copy;
use std::path::Path;
use reqwest::blocking::Client;

pub fn download_file(url: &str, destination: &str) -> Result<(), String> {
    // Make the GET request
    let response = match Client::new().get(url).send() {
        Ok(response) => response,
        Err(_) => return Err("Failed to get file".to_string()),
    };

    // Check if the response was successful
    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()));
    }

    // Read the response body as bytes
    let content = match response.bytes() {
        Ok(content) => content,
        Err(error) => return Err(format!("Failed to read response: {}", error)),
    };

    // Create the file at the destination path
    let path = Path::new(destination);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(error) => return Err(format!("Failed to create file: {}", error)),
    };

    // Copy the content to the file
    match copy(&mut content.as_ref(), &mut file) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Failed to write to file: {}", error)),
    }
}