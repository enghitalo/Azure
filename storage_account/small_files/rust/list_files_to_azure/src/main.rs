use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::Sha256;
use base64::prelude::*;
use xml::reader::{EventReader, XmlEvent};

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

    let prefix = std::env::var("PREFIX").unwrap_or_default();

    let api_version = "2019-12-12";

    // Create the request URL with filters
    // https://learn.microsoft.com/en-us/rest/api/storageservices/list-blobs?tabs=microsoft-entra-id#uri-parameters
    let url = format!(
        "https://{}.blob.core.windows.net/{}/?comp=list&restype=container&prefix={}",
        account_name, container_name, prefix
    );

    // Generate the current date in the required format
    let request_time_str = chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    // Create the string to sign for GET request
    // https://learn.microsoft.com/en-us/rest/api/storageservices/list-blobs?tabs=microsoft-entra-id#uri-parameters
    // https://learn.microsoft.com/en-us/rest/api/storageservices/authorize-with-shared-key
    let string_to_sign = format!(
        "GET\n\n\n\n\n\n\n\n\n\n\n\nx-ms-date:{}\nx-ms-version:{}\n/{}/{}/\ncomp:list\nprefix:{}\nrestype:container",
        request_time_str, api_version, account_name, container_name, prefix
    );

    // Generate the signature
    let signature_b64 = generate_signature_b64(&account_key, &string_to_sign)?;

    // Create the authorization header
    let authorization_header = format!("SharedKey {}:{}", account_name, signature_b64);

    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert("x-ms-date", HeaderValue::from_str(&request_time_str)?);
    headers.insert("x-ms-version", HeaderValue::from_static(api_version));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&authorization_header)?,
    );

    // Create a reqwest client and send the GET request
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?;

    // Check the response
    if response.status().is_success() {
        let body = response.text().await?;
        let mut parser = EventReader::from_str(&body);

        // Parse the XML response
        while let Ok(event) = parser.next() {
            match event {
                XmlEvent::StartElement { name, .. } if name.local_name == "Name" => {
                    if let Ok(XmlEvent::Characters(blob_name)) = parser.next() {
                        println!("Blob: {}", blob_name);
                    }
                }
                XmlEvent::EndDocument => break,
                _ => {}
            }
        }
    } else {
        println!("Failed to list blobs.");
        println!("Status code: {}", response.status());
        println!("Response: {:?}", response.text().await?);
    }

    Ok(())
}
