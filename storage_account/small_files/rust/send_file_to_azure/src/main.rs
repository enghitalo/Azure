use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use sha2::Sha256;
use std::env;
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

// cargo run -- <account_name> <account_key> <container_name> <blob_name> <file_path>
// cargo run -- <account_name> <account_key> <container_name> <blob_name> <file_path>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::args().len() < 6 {
        eprintln!("Usage: cargo run -- <account_name> <account_key> <container_name> <blob_name> <file_path>");
        return Err("Invalid number of arguments".into());
    }

    // Configuration
    let account_name = env::args().nth(1).expect("Missing account name argument");
    // println!("account_name: {}", account_name);

    let account_key = env::args().nth(2).expect("Missing account key argument");
    // println!("account_key: {}", account_key);

    let container_name = env::args().nth(3).expect("Missing container name argument");
    // println!("container_name: {}", container_name);

    let blob_name = env::args().nth(4).expect("Missing blob name argument");
    // println!("blob_name: {}", blob_name);

    let file_path = env::args().nth(5).expect("Missing file path argument");

    println!("file_path: {}", file_path);

    let api_version = "2019-12-12";

    // Read the file
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
            return Err(err.into());
        }
    };

    // Holds the data of the file to be sent.
    let mut file_data = Vec::new();

    file.read_to_end(&mut file_data)?;

    if file_data.len() == 0 {
        eprintln!("File '{}' is empty", file_path);
        return Err("File is empty".into());
    }

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
    let signature_b64 =
        generate_signature_b64(&account_key, &string_to_sign).unwrap_or_else(|err| {
            eprintln!("Failed to generate signature: {}", err);
            String::new()
        });

    // Create the authorization header
    let authorization_header = format!("SharedKey {}:{}", account_name, signature_b64);

    // println!("Authorization header: {}", authorization_header);
    // println!("String to sign: {}", string_to_sign);
    // println!("Request time: {}", request_time_str);
    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-ms-date",
        HeaderValue::from_str(&request_time_str).unwrap_or_else(|_| HeaderValue::from_static("")),
    );
    headers.insert("x-ms-version", HeaderValue::from_static("2019-12-12"));
    headers.insert("x-ms-blob-type", HeaderValue::from_static("BlockBlob"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&authorization_header)
            .unwrap_or_else(|_| HeaderValue::from_static("")),
    );
    headers.insert(CONTENT_LENGTH, HeaderValue::from(file_data.len() as u64));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );

    // Create a reqwest client and send the PUT request
    let client = reqwest::Client::new();

    let request = client.put(&url).headers(headers).body(file_data).build();

    let request = match request {
        Ok(request) => request,
        Err(err) => {
            eprintln!("Failed to build request: {}", err);
            return Err(err.into());
        }
    };

    // print!("request {:?}", request);

    let response = client.execute(request).await?;

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
