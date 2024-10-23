use crate::{handlers::google_drive_handler::{get_file_by_id, get_list_files_in_folder, get_list_folders, upload_pdf_file}, services::google_drive_service::GoogleDriveService};
use actix_web::web;

pub fn drive_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/drive")
            .app_data(web::Data::new(GoogleDriveService))
            .route("/list-folders", web::get().to(get_list_folders::<GoogleDriveService>))
            .route("/files", web::get().to(get_list_files_in_folder::<GoogleDriveService>))
            .route("/files/{file_id}", web::get().to(get_file_by_id::<GoogleDriveService>))
            .route("/files", web::post().to(upload_pdf_file::<GoogleDriveService>))
    );
}
