use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde::Serialize;
use crate::api::google_drive::list_folders;
use crate::config::Config;
use std::future::Future;
use std::error::Error;
use std::pin::Pin;

#[derive(Serialize)]
pub struct FolderInfo {
    id: Option<String>,
    name: Option<String>,
}
pub trait DriveService {
    fn list_folders<'a>(
        &'a self,
        token: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FolderInfo>, Box<dyn Error>>> + 'a>>;
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
}

pub async fn get_list_folders<T: DriveService>(
    req: HttpRequest,
    config: web::Data<Config>,
    drive_service: web::Data<T>,
) -> impl Responder {

    if let Some(token) = req.headers().get("Authorization") {
        let token_str = token.to_str().unwrap_or("").replace("Bearer ", "");
        
        match drive_service.list_folders(&token_str, &config).await {
            Ok(folders) => HttpResponse::Ok().json(folders),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error listing folders: {:?}", err)),
        }
    } else {
        HttpResponse::BadRequest().body("Authorization token missing")
    }
}
