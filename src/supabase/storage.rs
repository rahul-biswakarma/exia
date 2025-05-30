use super::SupabaseClient;
use reqwest::multipart::{Form, Part};
use serde_json::Value;

pub struct StorageService;

impl StorageService {
    // Upload a file to Supabase Storage
    pub async fn upload_file(
        client: &SupabaseClient,
        bucket: &str,
        path: &str,
        file_data: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<String, String> {
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            client.config.url, bucket, path
        );

        let mut req = client.authenticated_request(reqwest::Method::POST, &url);

        if let Some(content_type) = content_type {
            req = req.header("Content-Type", content_type);
        }

        let response = req
            .body(file_data)
            .send()
            .await
            .map_err(|e| format!("Failed to upload file: {}", e))?;

        if response.status().is_success() {
            let public_url = format!(
                "{}/storage/v1/object/public/{}/{}",
                client.config.url, bucket, path
            );
            Ok(public_url)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Upload failed: {}", error_text))
        }
    }

    // Get a signed URL for uploading
    pub async fn get_signed_upload_url(
        client: &SupabaseClient,
        bucket: &str,
        path: &str,
        expires_in: u32, // seconds
    ) -> Result<String, String> {
        let url = format!(
            "{}/storage/v1/object/sign/{}/{}",
            client.config.url, bucket, path
        );

        let body = serde_json::json!({
            "expiresIn": expires_in
        });

        let response = client
            .authenticated_request(reqwest::Method::POST, &url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to get signed URL: {}", e))?;

        if response.status().is_success() {
            let data: Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            data.get("signedURL")
                .and_then(|url| url.as_str())
                .map(|s| s.to_string())
                .ok_or("No signed URL in response".to_string())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Failed to get signed URL: {}", error_text))
        }
    }

    // Delete a file from storage
    pub async fn delete_file(
        client: &SupabaseClient,
        bucket: &str,
        path: &str,
    ) -> Result<(), String> {
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            client.config.url, bucket, path
        );

        let response = client
            .authenticated_request(reqwest::Method::DELETE, &url)
            .send()
            .await
            .map_err(|e| format!("Failed to delete file: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Delete failed: {}", error_text))
        }
    }

    // List files in a bucket
    pub async fn list_files(
        client: &SupabaseClient,
        bucket: &str,
        prefix: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<StorageFile>, String> {
        let mut url = format!("{}/storage/v1/object/list/{}", client.config.url, bucket);

        let mut query_params = Vec::new();
        if let Some(prefix) = prefix {
            query_params.push(format!("prefix={}", prefix));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }

        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
        }

        let response = client
            .authenticated_request(reqwest::Method::GET, &url)
            .send()
            .await
            .map_err(|e| format!("Failed to list files: {}", e))?;

        if response.status().is_success() {
            let files: Vec<StorageFile> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(files)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("List failed: {}", error_text))
        }
    }

    // Get public URL for a file
    pub fn get_public_url(client: &SupabaseClient, bucket: &str, path: &str) -> String {
        format!(
            "{}/storage/v1/object/public/{}/{}",
            client.config.url, bucket, path
        )
    }

    // Helper method to upload UI schema screenshots/previews
    pub async fn upload_schema_preview(
        client: &SupabaseClient,
        schema_id: &str,
        image_data: Vec<u8>,
    ) -> Result<String, String> {
        let path = format!("previews/{}.png", schema_id);
        Self::upload_file(client, "ui-previews", &path, image_data, Some("image/png")).await
    }

    // Helper method to upload user avatars
    pub async fn upload_avatar(
        client: &SupabaseClient,
        user_id: &str,
        image_data: Vec<u8>,
        file_extension: &str,
    ) -> Result<String, String> {
        let path = format!("avatars/{}.{}", user_id, file_extension);
        let content_type = match file_extension {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            _ => "application/octet-stream",
        };

        Self::upload_file(client, "avatars", &path, image_data, Some(content_type)).await
    }
}

// Data structures for storage
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct StorageFile {
    pub name: String,
    pub id: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub last_accessed_at: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// File upload helper for web
#[cfg(target_arch = "wasm32")]
pub mod web_upload {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{File, FileReader};

    pub async fn read_file_as_bytes(file: &File) -> Result<Vec<u8>, String> {
        let file_reader = FileReader::new().map_err(|_| "Failed to create FileReader")?;

        // Set up the file reader to read as array buffer
        file_reader
            .read_as_array_buffer(file)
            .map_err(|_| "Failed to read file")?;

        // Wait for the read operation to complete
        let promise = js_sys::Promise::new(&mut |resolve, _reject| {
            let reader_clone = file_reader.clone();
            let onload = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                let result = reader_clone.result().unwrap();
                resolve.call1(&JsValue::NULL, &result).unwrap();
            }) as Box<dyn FnMut(_)>);

            file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
        });

        let result = JsFuture::from(promise)
            .await
            .map_err(|_| "Failed to read file data")?;

        // Convert the ArrayBuffer to Vec<u8>
        let array_buffer = result
            .dyn_into::<js_sys::ArrayBuffer>()
            .map_err(|_| "Failed to convert to ArrayBuffer")?;
        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let mut bytes = vec![0; uint8_array.length() as usize];
        uint8_array.copy_to(&mut bytes);

        Ok(bytes)
    }

    // Helper to get file extension from file name
    pub fn get_file_extension(filename: &str) -> Option<&str> {
        filename.split('.').last()
    }
}
