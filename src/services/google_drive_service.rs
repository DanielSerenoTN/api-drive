use serde::Serialize;
use utoipa::{IntoParams, ToSchema};
use crate::api::google_drive::{download_pdf, list_files_from_folder, list_folders, upload_pdf_file, initialize_resumable_upload};
use crate::config::Config;
use anyhow::{Result, Context};
use std::future::Future;
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
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FolderInfo>>> + Send + 'a>>;

    fn list_files_in_folder<'a>(
        &'a self,
        token: &'a str,
        folder_id: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileInfo>>> + Send + 'a>>;

    fn download_pdf<'a>(
        &'a self,
        token: &'a str,
        file_id: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + 'a>>;

    fn upload_pdf<'a>(
        &'a self,
        token: &'a str,
        resumable_url: &'a str,
        file_content: Vec<u8>,
        start_byte: Option<u64>
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;

    fn initialize_resumable_upload<'a>(
        &'a self,
        token: &'a str,
        folder_id: &'a str,
        file_name: &'a str,
        config: &'a Config,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;
}

pub struct GoogleDriveService;

impl DriveService for GoogleDriveService {
    fn list_folders<'a>(
        &'a self,
        token: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FolderInfo>>> + Send + 'a>> {
        Box::pin(async move {
            list_folders(token, config)
                .await
                .with_context(|| "Failed to list folders")
                .map(|folders| {
                    folders
                        .into_iter()
                        .map(|folder| FolderInfo {
                            id: folder.id,
                            name: folder.name,
                        })
                        .collect()
                })
        })
    }

    fn list_files_in_folder<'a>(
        &'a self,
        token: &'a str,
        folder_id: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileInfo>>> + Send + 'a>> {
        Box::pin(async move {
            list_files_from_folder(token, folder_id, config)
                .await
                .with_context(|| format!("Failed to list files in folder: {}", folder_id))
                .map(|files| {
                    files
                        .into_iter()
                        .map(|file| FileInfo {
                            id: file.id,
                            name: file.name,
                            mime_type: file.mime_type,
                            created_time: file.created_time,
                        })
                        .collect()
                })
        })
    }

    fn download_pdf<'a>(
        &'a self,
        token: &'a str,
        file_id: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + 'a>> {
        Box::pin(async move {
            download_pdf(token, file_id, config)
                .await
                .with_context(|| format!("Failed to download PDF with ID: {}", file_id))
        })
    }

    fn upload_pdf<'a>(
        &'a self,
        token: &'a str,
        resumable_url: &'a str,
        file_content: Vec<u8>,
        start_byte: Option<u64>
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            upload_pdf_file(token, resumable_url, file_content, start_byte)
                .await
                .with_context(|| format!("Failed to upload file chunk to URL: {}", resumable_url))
        })
    }

    fn initialize_resumable_upload<'a>(
        &'a self,
        token: &'a str,
        folder_id: &'a str,
        file_name: &'a str,
        config: &'a Config,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            initialize_resumable_upload(token, folder_id, file_name, config)
                .await
                .with_context(|| format!("Failed to initialize resumable upload for file: {}", file_name))
        })
    }
}
