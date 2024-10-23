use reqwest::{Client, multipart};
use serde::Deserialize;
use crate::config::Config;
use std::error::Error;

#[derive(Deserialize)]
pub struct File {
    pub id: Option<String>,
    pub name: Option<String>,
    pub mime_type: Option<String>,
    pub created_time: Option<String>,
}

#[derive(Deserialize)]
pub struct FileList {
    pub files: Vec<File>,
}

pub async fn list_folders(token: &str, config: &Config) -> Result<Vec<File>, Box<dyn Error>> {
    let client = Client::new();
    
    let folder_url = format!("{}?q=mimeType='application/vnd.google-apps.folder'&fields=files(id,name)", &config.drive_api_base_url);

    let response = client
        .get(&folder_url)
        .bearer_auth(token)
        .send()
        .await?;

    let file_list = response.json::<FileList>().await?;
    
    if !file_list.files.is_empty() {
        Ok(file_list.files)
    } else {
        println!("No folders found.");
        Ok(vec![])
    }
}

pub async fn list_files_from_folder(token: &str, folder_id: &str, config: &Config) -> Result<Vec<File>, Box<dyn Error>> {
    let client = Client::new();
    
    let files_url = format!(
        "{}?q='{}' in parents&fields=files(id,name,mimeType,createdTime)", 
        &config.drive_api_base_url, folder_id
    );

    let response = client
        .get(&files_url)
        .bearer_auth(token)
        .send()
        .await?;

    let file_list = response.json::<FileList>().await?;
    
    if !file_list.files.is_empty() {
        Ok(file_list.files)
    } else {
        println!("No files found in folder.");
        Ok(vec![])
    }
}


pub async fn download_pdf(token: &str, file_id: &str, config: &Config) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = Client::new();

    let file_url = format!(
        "{}/{}?alt=media", 
        &config.drive_api_base_url, file_id
    );

    let response = client
        .get(&file_url)
        .bearer_auth(token)
        .send()
        .await?;

    if response.status().is_success() {
        let file_bytes = response.bytes().await?;

        Ok(file_bytes.to_vec())
    } else {
        Err(Box::from(format!("Failed to download file: {}", response.status())))
    }
}

pub async fn upload_pdf_file(
    token: &str,
    folder_id: &str,
    file_name: String,
    file_content: Vec<u8>,
    config: &Config
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let upload_url = format!("{}?uploadType=multipart", &config.drive_upload_url);

    let metadata = format!(
        r#"{{
            "name": "{}",
            "parents": ["{}"]
        }}"#,
        file_name, folder_id
    );

    let form = multipart::Form::new()
        .part("metadata", multipart::Part::text(metadata).mime_str("application/json")?)
        .part("file", multipart::Part::bytes(file_content).file_name(file_name.clone()).mime_str("application/pdf")?);

    let response = client
        .post(&upload_url)
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await?;

    if response.status().is_success() {
        let json_response: serde_json::Value = response.json().await?;

        let file_id = json_response["id"].as_str().unwrap_or("").to_string();

        Ok(file_id)
    } else {
        Err(Box::from(format!("Failed to upload file: {}", response.status())))
    }
}


