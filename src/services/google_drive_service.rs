use serde::Serialize;
use crate::api::google_drive::{list_folders, list_files_from_folder};
use crate::config::Config;
use std::future::Future;
use std::error::Error;
use std::pin::Pin;

#[derive(Serialize)]
pub struct FolderInfo {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Serialize)]
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
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FolderInfo>, Box<dyn Error>>> + 'a>>;

    fn list_files_in_folder<'a>(
        &'a self,
        token: &'a str,
        folder_id: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileInfo>, Box<dyn Error>>> + 'a>>;
}

pub struct GoogleDriveService;

impl DriveService for GoogleDriveService {
    fn list_folders<'a>(
        &'a self,
        token: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FolderInfo>, Box<dyn Error>>> + 'a>> {
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
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileInfo>, Box<dyn Error>>> + 'a>> {
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
}

