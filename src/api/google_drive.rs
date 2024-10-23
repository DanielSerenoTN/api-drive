use reqwest::Client;
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
