use serde::Serialize;
use utoipa::{IntoParams, ToSchema};
use crate::api::google_drive::{download_pdf, list_files_from_folder, list_folders, upload_pdf_file, initialize_resumable_upload};
use crate::config::Config;
use std::future::Future;
use std::error::Error;
use std::pin::Pin;

#[derive(Serialize, IntoParams, ToSchema)]
pub struct FolderInfo {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, IntoParams, ToSchema)]
pub struct FileInfo {
    id: Option<String>,
    name: Option<String>,
    mime_type: Option<String>,
    created_time: Option<String>,
}

pub trait DriveService {
    fn list_folders<'a>(
        &'a self,
        token: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FolderInfo>, Box<dyn Error + Send + Sync>>> + Send + 'a>>;

    fn list_files_in_folder<'a>(
        &'a self,
        token: &'a str,
        folder_id: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileInfo>, Box<dyn Error + Send + Sync>>> + Send + 'a>>;

    fn download_pdf<'a>(
        &'a self,
        token: &'a str,
        file_id: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, Box<dyn Error + Send + Sync>>> + Send + 'a>>;

    fn upload_pdf<'a>(
        &'a self,
        token: &'a str,
        resumable_url: &'a str,
        file_content: Vec<u8>,
        start_byte: Option<u64>
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error + Send + Sync>>> + Send + 'a>>;

    fn initialize_resumable_upload<'a>(
        &'a self,
        token: &'a str,
        folder_id: &'a str,
        file_name: &'a str,
        config: &'a Config,
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error + Send + Sync>>> + Send + 'a>>;

}

pub struct GoogleDriveService;
impl DriveService for GoogleDriveService {
    fn list_folders<'a>(
        &'a self,
        token: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FolderInfo>, Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move {
            match list_folders(token, config).await {
                Ok(folders) => {
                    let folder_info: Vec<FolderInfo> = folders
                        .into_iter()
                        .map(|folder| FolderInfo {
                            id: folder.id,
                            name: folder.name,
                        })
                        .collect();
                    Ok(folder_info)
                }
                Err(e) => Err(Box::from(format!("Error getting folders: {:?}", e))),
            }
        })
    }

    fn list_files_in_folder<'a>(
        &'a self,
        token: &'a str,
        folder_id: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileInfo>, Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move {
            match list_files_from_folder(token, folder_id, config).await {
                Ok(files) => {
                    let file_info: Vec<FileInfo> = files
                        .into_iter()
                        .map(|file| FileInfo {
                            id: file.id,
                            name: file.name,
                            mime_type: file.mime_type,
                            created_time: file.created_time,
                        })
                        .collect();
                    Ok(file_info)
                }
                Err(e) => Err(Box::from(format!("Error getting files: {:?}", e))),
            }
        })
    }

    fn download_pdf<'a>(
        &'a self,
        token: &'a str,
        file_id: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move {
            match download_pdf(token, file_id, config).await {
                Ok(file) => {
                    Ok(file)
                }
                Err(e) => {
                    Err(Box::from(format!("Error downloading file: {:?}", e)))
                }
            }
        })
    }

    fn upload_pdf<'a>(
        &'a self,
        token: &'a str,
        resumable_url: &'a str,
        file_content: Vec<u8>,
        start_byte: Option<u64>
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move {
            match upload_pdf_file(token, resumable_url, file_content, start_byte).await {
                Ok(file_id) => Ok(file_id),
                Err(e) => Err(Box::from(format!("Error uploading file: {:?}", e))),
            }
        })
    }

    fn initialize_resumable_upload<'a>(
        &'a self,
        token: &'a str,
        folder_id: &'a str,
        file_name: &'a str,
        config: &'a Config,
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move {
            match initialize_resumable_upload(token, folder_id, file_name, config).await {
                Ok(url) => Ok(url),
                Err(e) => Err(Box::from(format!("Error initializing resumable upload: {:?}", e))),
            }
        })
    }
        
}
    
