use crate::{handlers::google_drive_handler::{get_list_files_in_folder, get_list_folders}, services::google_drive_service::GoogleDriveService};
use actix_web::web;

pub fn drive_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/drive")
            .app_data(web::Data::new(GoogleDriveService))
            .route("/list-folders", web::get().to(get_list_folders::<GoogleDriveService>))
            .route("/files", web::get().to(get_list_files_in_folder::<GoogleDriveService>))
    );
}
