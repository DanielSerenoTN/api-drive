use std::future::Future;
use std::pin::Pin;
use anyhow::Result;
use api_drive::{config::Config, services::google_drive_service::{DriveService, FileInfo, FolderInfo}};

pub struct MockGoogleDriveService;

impl DriveService for MockGoogleDriveService {
    fn list_folders<'a>(
        &'a self,
        _token: &'a str,
        _config: &'a Config,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FolderInfo>>> + Send + 'a>> {
        Box::pin(async move {
            Ok(vec![
                FolderInfo {
                    id: Some("1".to_string()),
                    name: Some("Folder 1".to_string()),
                },
                FolderInfo {
                    id: Some("2".to_string()),
                    name: Some("Folder 2".to_string()),
                },
            ])
        })
    }

    fn list_files_in_folder<'a>(
        &'a self,
        _token: &'a str,
        _folder_id: &'a str,
        _config: &'a Config,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileInfo>>> + Send + 'a>> {
        Box::pin(async move {
            Ok(vec![
                FileInfo {
                    id: Some("file1".to_string()),
                    name: Some("File 1".to_string()),
                    mime_type: Some("application/pdf".to_string()),
                    created_time: Some("2024-10-23T10:00:00Z".to_string()),
                },
                FileInfo {
                    id: Some("file2".to_string()),
                    name: Some("File 2".to_string()),
                    mime_type: Some("application/pdf".to_string()),
                    created_time: Some("2024-10-24T11:00:00Z".to_string()),
                },
            ])
        })
    }

    fn download_pdf<'a>(
        &'a self,
        _token: &'a str,
        _file_id: &'a str,
        _config: &'a Config,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + 'a>> {
        Box::pin(async move {
            Ok(vec![0x25, 0x50, 0x44, 0x46])
        })
    }

    fn upload_pdf<'a>(
        &'a self,
        _token: &'a str,
        _resumable_url: &'a str,
        _file_content: Vec<u8>,
        _start_byte: Option<u64>,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            Ok("mock_file_id".to_string())
        })
    }

    fn initialize_resumable_upload<'a>(
        &'a self,
        _token: &'a str,
        _folder_id: &'a str,
        _file_name: &'a str,
        _config: &'a Config,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            Ok("mock_resumable_url".to_string())
        })
    }
}
