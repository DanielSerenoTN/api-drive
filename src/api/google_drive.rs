use actix_web::http::header::HeaderMap;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{header::RANGE, Client};
use serde::Deserialize;
use serde_json::json;
use crate::config::Config;
use anyhow::{Result, Context};

#[derive(Deserialize)]
pub struct File {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
}

#[derive(Deserialize)]
pub struct FileList {
    pub files: Vec<File>,
}

pub async fn list_folders(token: &str, config: &Config) -> Result<Vec<File>> {
    let client = Client::new();
    let folder_url = format!("{}?q=mimeType='application/vnd.google-apps.folder'&fields=files(id,name)", &config.drive_api_base_url);

    let response = client
        .get(&folder_url)
        .bearer_auth(token)
        .send()
        .await
        .context("Failed to send request to list folders")?;

    let file_list = response.json::<FileList>().await
        .context("Failed to parse response as FileList")?;

    if !file_list.files.is_empty() {
        Ok(file_list.files)
    } else {
        println!("No folders found.");
        Ok(vec![])
    }
}

pub async fn list_files_from_folder(token: &str, folder_id: &str, config: &Config) -> Result<Vec<File>> {
    let client = Client::new();
    let files_url = format!(
        "{}?q='{}' in parents&fields=files(id,name,mimeType,createdTime)", 
        &config.drive_api_base_url, folder_id
    );

    let response = client
        .get(&files_url)
        .bearer_auth(token)
        .send()
        .await
        .context(format!("Failed to send request to list files in folder '{}'", folder_id))?;

    let file_list = response.json::<FileList>().await
        .context("Failed to parse response as FileList")?;

    if !file_list.files.is_empty() {
        Ok(file_list.files)
    } else {
        println!("No files found in folder.");
        Ok(vec![])
    }
}

pub async fn download_pdf(token: &str, file_id: &str, config: &Config) -> Result<Vec<u8>> {
    let client = Client::new();
    let file_url = format!("{}/{}?alt=media", &config.drive_api_base_url, file_id);

    let response = client
        .get(&file_url)
        .bearer_auth(token)
        .send()
        .await
        .context(format!("Failed to send request to download file '{}'", file_id))?;

    if response.status().is_success() {
        let file_bytes = response.bytes().await
            .context("Failed to read file bytes from response")?;

        Ok(file_bytes.to_vec())
    } else {
        Err(anyhow::anyhow!("Failed to download file: {}", response.status()))
    }
}

pub async fn upload_pdf_file(
    token: &str,
    resumable_url: &str,
    file_content: Vec<u8>,
    start_byte: Option<u64>
) -> Result<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    if let Some(start) = start_byte {
        headers.insert(RANGE, format!("bytes={}-", start).parse()?);
    }

    let response = client
        .put(resumable_url)
        .bearer_auth(token)
        .headers(headers.into())
        .body(file_content)
        .send()
        .await
        .context("Failed to upload PDF file chunk")?;

    if response.status().is_success() {
        let json_response: serde_json::Value = response.json().await
            .context("Failed to parse response as JSON after uploading chunk")?;
        let file_id = json_response["id"].as_str().unwrap_or("").to_string();
        Ok(file_id)
    } else {
        Err(anyhow::anyhow!("Failed to upload file: {}", response.status()))
    }
}

pub async fn initialize_resumable_upload(
    token: &str,
    folder_id: &str,
    file_name: &str,
    config: &Config,
) -> Result<String> {
    let client = Client::new();
    let upload_url = format!("{}?uploadType=resumable", &config.drive_upload_url);

    let metadata = json!({
        "name": file_name,
        "parents": [folder_id]
    });

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

    let response = client
        .post(&upload_url)
        .headers(headers.into())
        .json(&metadata)
        .send()
        .await
        .context("Failed to initialize resumable upload")?;

    if response.status().is_success() {
        if let Some(resumable_url) = response.headers().get("Location") {
            Ok(resumable_url.to_str()?.to_string())
        } else {
            Err(anyhow::anyhow!("Failed to get resumable upload URL"))
        }
    } else {
        Err(anyhow::anyhow!("Failed to initialize upload: {}", response.status()))
    }
}
