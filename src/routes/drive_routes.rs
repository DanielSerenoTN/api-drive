use crate::services::google_drive_service::{GoogleDriveService, get_list_folders};
use actix_web::web;

pub fn drive_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/drive")
            .app_data(web::Data::new(GoogleDriveService))
            .route("/list-folders", web::get().to(get_list_folders::<GoogleDriveService>))
    );
}
