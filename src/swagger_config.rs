use utoipa::{openapi::security::{Http, HttpAuthScheme, SecurityScheme}, Modify, OpenApi};
use crate::services::{auth_service::AuthCallbackQuery, google_drive_service::{FolderInfo, FileInfo}};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth_handler::get_auth_url,
        crate::handlers::auth_handler::auth_callback,
        crate::handlers::google_drive_handler::get_list_folders,
        crate::handlers::google_drive_handler::get_list_files_in_folder,
        crate::handlers::google_drive_handler::download_pdf_file_by_id,
        crate::handlers::google_drive_handler::upload_pdf_file,
    ),
    modifiers(&SecurityAddon),
    components(schemas(AuthCallbackQuery, FolderInfo, FileInfo)),
    tags(
        (name = "auth", description = "Authentication related endpoints"),
        (name = "drive", description = "Google Drive API related endpoints")
    ),
    info(description = "This API allows users to interact with their Google Drive account through secure transactions authenticated with OAuth 2.0.")
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}
