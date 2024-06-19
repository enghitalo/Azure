use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use sha2::Sha256;
use std::fs::File;
use std::io::Read;

use base64::prelude::*;

/// Generates a Base64-encoded signature for the given string to sign using the provided account key.
///
/// # Arguments
///
/// * `account_key` - The storage account key.
/// * `string_to_sign` - The string to sign for authentication purposes.
///
/// # Returns
///
/// A `Result` containing the Base64-encoded signature if successful, or a boxed `dyn std::error::Error` if an error occurred.
fn generate_signature_b64(
    account_key: &str,
    string_to_sign: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Decode the storage account key and create the HMAC-SHA256 hash
    let decoded_key = BASE64_STANDARD.decode(account_key)?;

    let mut mac = Hmac::<Sha256>::new_from_slice(&decoded_key)?;
    mac.update(string_to_sign.as_bytes());
    let signature = mac.finalize().into_bytes();

    // Encode the signature to Base64
    let signature_b64 = BASE64_STANDARD.encode(&signature);

    Ok(signature_b64)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration
    let account_name =
        std::env::var("ACCOUNT_NAME").expect("Missing ACCOUNT_NAME environment variable");

    let account_key =
        std::env::var("ACCOUNT_KEY").expect("Missing ACCOUNT_KEY environment variable");

    let container_name =
        std::env::var("CONTAINER_NAME").expect("Missing CONTAINER_NAME environment variable");

    let blob_name = std::env::var("BLOB_NAME").expect("Missing BLOB_NAME environment variable");

    let file_path = std::env::var("FILE_PATH").expect("Missing FILE_PATH environment variable");

    let api_version = "2019-12-12";

    // Read the file
    let mut file = File::open(file_path)?;

    // Holds the data of the file to be sent.
    let mut file_data = Vec::new();

    file.read_to_end(&mut file_data)?;

    // Create the request URL
    let url = format!(
        "https://{}.blob.core.windows.net/{}/{}",
        account_name, container_name, blob_name
    );

    // Generate the current date in the required format
    let request_time_str = format!(
        "{} GMT",
        chrono::Utc::now()
            .format("%a, %d %b %Y %H:%M:%S")
            .to_string()
    );

    // Create the string to sign
    // The string that will be signed for authentication purposes.
    // It is constructed using various components such as the file data length, request time, API version,
    // account name, container name, and blob name.
    let string_to_sign = format!(
        "PUT\n\n\n{}\n\napplication/octet-stream\n\n\n\n\n\n\nx-ms-blob-type:BlockBlob\nx-ms-date:{}\nx-ms-version:{}\n/{}/{}/{}",
        file_data.len(), request_time_str, api_version, account_name, container_name, blob_name
    );

    // Generate the signature
    let signature_b64 = generate_signature_b64(&account_key, &string_to_sign)?;

    // Create the authorization header
    let authorization_header = format!("SharedKey {}:{}", account_name, signature_b64);

    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert("x-ms-date", HeaderValue::from_str(&request_time_str)?);
    headers.insert("x-ms-version", HeaderValue::from_static("2019-12-12"));
    headers.insert("x-ms-blob-type", HeaderValue::from_static("BlockBlob"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&authorization_header)?,
    );
    headers.insert(CONTENT_LENGTH, HeaderValue::from(file_data.len() as u64));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );

    // Create a reqwest client and send the PUT request
    let client = reqwest::Client::new();
    let response = client
        .put(&url)
        .headers(headers)
        .body(file_data)
        .send()
        .await?;

    // Check the response
    if response.status().is_success() {
        println!("File uploaded successfully.");
    } else {
        println!("Failed to upload file.");
        println!("Status code: {}", response.status());
    }
    // println!("Response: {:?}", response.text().await?);

    Ok(())
}
