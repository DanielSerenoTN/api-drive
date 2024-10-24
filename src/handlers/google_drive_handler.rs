use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use futures::StreamExt;
use serde::Deserialize;
use utoipa::ToSchema;
use std::time::Instant;
use crate::{config::Config, services::google_drive_service::{DriveService, FileInfo, FolderInfo}};
use anyhow::Context;

#[utoipa::path(
    get,
    path = "/drive/list-folders",
    responses(
        (status = 200, description = "List of folders in the user's Google Drive", body = [FolderInfo]),
        (status = 400, description = "Authorization token missing or invalid"),
        (status = 500, description = "Internal server error while listing folders.")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "drive"
)]
pub async fn get_list_folders<T: DriveService>(
    req: HttpRequest,
    config: web::Data<Config>,
    drive_service: web::Data<T>,
) -> impl Responder {
    let token = match req.headers().get("Authorization") {
        Some(token) => token.to_str().ok().map(|t| t.replace("Bearer ", "")),
        None => None,
    };

    if let Some(token_str) = token {
        match drive_service.list_folders(&token_str, &config).await.context("Failed to list folders") {
            Ok(folders) => HttpResponse::Ok().json(folders),
            Err(err) => {
                eprintln!("Error listing folders: {:?}", err);
                HttpResponse::InternalServerError().body(format!("Error listing folders: {:?}", err))
            }
        }
    } else {
        HttpResponse::BadRequest().body("Authorization token missing or invalid")
    }
}

#[utoipa::path(
    get,
    path = "/drive/files",
    params(
        ("folder_id" = String, Query, description = "ID of the folder from which to list files")
    ),
    responses(
        (status = 200, description = "List of files in the specified Google Drive folder", body = [FileInfo]),
        (status = 400, description = "Authorization token missing or invalid, or folder ID missing"),
        (status = 500, description = "Internal server error while listing files.")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "drive"
)]
pub async fn get_list_files_in_folder<T: DriveService>(
    req: HttpRequest,
    config: web::Data<Config>,
    drive_service: web::Data<T>,
) -> impl Responder {
    let token = match req.headers().get("Authorization") {
        Some(token) => token.to_str().ok().map(|t| t.replace("Bearer ", "")),
        None => None,
    };

    if let Some(token_str) = token {
        let query_params = req.query_string();

        let folder_id = query_params
            .split('&')
            .find_map(|param| {
                let mut kv = param.split('=');
                if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                    if key == "folder_id" {
                        return Some(value);
                    }
                }
                None
            });

        let folder_id = match folder_id {
            Some(id) => id,
            None => return HttpResponse::BadRequest().body("Missing folder_id in query parameters"),
        };

        match drive_service
            .list_files_in_folder(&token_str, folder_id, &config)
            .await
            .context("Failed to list files in folder")
        {
            Ok(files) => HttpResponse::Ok().json(files),
            Err(err) => {
                eprintln!("Error listing files: {:?}", err);
                HttpResponse::InternalServerError().body(format!("Error listing files: {:?}", err))
            }
        }
    } else {
        HttpResponse::BadRequest().body("Authorization token missing or invalid")
    }
}

#[derive(Debug, Deserialize)]
pub struct FileId {
    file_id: String,
}

#[utoipa::path(
    get,
    path = "/drive/files/{file_id}",
    params(
        ("file_id" = String, Path, description = "ID of the pdf file to be downloaded")
    ),
    responses(
        (status = 200, description = "File successfully downloaded", content_type = "application/pdf"),
        (status = 400, description = "Authorization token missing or invalid"),
        (status = 500, description = "Error downloading the file")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "drive"
)]
pub async fn download_pdf_file_by_id<T: DriveService>(
    file_id: web::Path<FileId>,
    req: HttpRequest,
    config: web::Data<Config>,
    drive_service: web::Data<T>,
) -> impl Responder {
    let token = match req.headers().get("Authorization") {
        Some(token) => token.to_str().ok().map(|t| t.replace("Bearer ", "")),
        None => None,
    };

    if let Some(token_str) = token {
        let file_id_str = &file_id.file_id;

        match drive_service
            .download_pdf(&token_str, file_id_str, &config)
            .await
            .context("Failed to download PDF file")
        {
            Ok(file) => HttpResponse::Ok().body(file),
            Err(err) => {
                eprintln!("Error downloading file: {:?}", err);
                HttpResponse::InternalServerError().body(format!("Error downloading file: {:?}", err))
            }
        }
    } else {
        HttpResponse::BadRequest().body("Authorization token missing or invalid")
    }
}

#[derive(ToSchema)]
pub struct FileUploadBody {
    pub _file: String,
}

#[utoipa::path(
    post,
    path = "/drive/files",
    request_body(content = FileUploadBody, description = "PDF file(multipart/form-data) to be uploaded"),
    params(
        ("folder_id" = Option<String>, Query, description = "ID of the folder where the PDF will be uploaded (defaults to root folder if not provided)")
    ),
    responses(
        (status = 200, description = "File uploaded successfully", body = String),
        (status = 400, description = "Authorization token missing or invalid"),
        (status = 500, description = "Internal server error while uploading file")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "drive"
)]
pub async fn upload_pdf_file<T: DriveService + Send + Sync + 'static>(
    req: HttpRequest,
    mut payload: Multipart,
    config: web::Data<Config>,
    drive_service: web::Data<T>,
) -> impl Responder {
    let token = match req.headers().get("Authorization") {
        Some(token) => token.to_str().ok().map(|t| t.replace("Bearer ", "")),
        None => None,
    };

    if let Some(token_str) = token {
        let mut file_name = String::new();
        let mut last_file_id = String::new();

        let folder_id = req.query_string()
            .split('&')
            .find_map(|param| {
                let mut kv = param.split('=');
                if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                    if key == "folder_id" {
                        return Some(value);
                    }
                }
                None
            })
            .unwrap_or("root");

        let resumable_url = match drive_service.get_ref()
            .initialize_resumable_upload(&token_str, folder_id, &file_name, &config)
            .await
            .context("Failed to initialize resumable upload")
        {
            Ok(url) => url,
            Err(e) => {
                eprintln!("Failed to initialize upload: {:?}", e);
                return HttpResponse::InternalServerError().body(format!("Failed to initialize upload: {:?}", e));
            },
        };

        while let Some(Ok(mut field)) = payload.next().await {
            let content_disposition = field.content_disposition();

            if let Some(name) = content_disposition.get_filename() {
                file_name = name.to_string();
                println!("Uploading file: {}", file_name);
            }

            while let Some(chunk) = field.next().await {
                match chunk {
                    Ok(data) => {
                        let chunk_size = data.len();
                        let start_time = Instant::now();

                        println!("Uploading chunk of size: {} bytes", chunk_size);

                        match drive_service.upload_pdf(&token_str, &resumable_url, data.to_vec(), None).await.context("Failed to upload file chunk") {
                            Ok(file_id) => {
                                let duration = start_time.elapsed();
                                println!(
                                    "Chunk of size {} bytes uploaded successfully in {:?} seconds",
                                    chunk_size,
                                    duration.as_secs_f64()
                                );
                                last_file_id = file_id;
                            },
                            Err(err) => {
                                eprintln!("Error uploading file chunk: {:?}", err);
                                return HttpResponse::InternalServerError().body("Error uploading file chunk");
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("Error reading file content");
                        return HttpResponse::InternalServerError().body("Error reading file content");
                    }
                }
            }
        }

        println!("File '{}' uploaded successfully with ID '{}'", file_name, last_file_id);

        HttpResponse::Ok().json(serde_json::json!({
            "file_name": file_name,
            "file_id": last_file_id
        }))
    } else {
        eprintln!("Authorization token missing or invalid");
        HttpResponse::BadRequest().body("Authorization token missing or invalid")
    }
}
