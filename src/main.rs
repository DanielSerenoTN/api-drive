mod config;
mod routes;
mod services;
mod api;
mod handlers;
mod middlewares;
use config::Config;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use middlewares::auth_guard::AuthGuard;
use services::{auth_service::AuthCallbackQuery, google_drive_service::{FolderInfo, FileInfo}};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

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
    components(schemas(AuthCallbackQuery, FolderInfo, FileInfo)),
    tags(
        (name = "auth", description = "Authentication related endpoints"),
        (name = "drive", description = "google drive api related endpoints")
    ),
    info(description = "This API allows users to interact with their Google Drive account through secure transactions authenticated with OAuth 2.0."),
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();
    let config_data = web::Data::new(config.clone());
    println!("Starting server at {}", config.serv_addrs);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header();

        App::new()
            .app_data(config_data.clone())
            .wrap(cors)
            .configure(routes::auth_routes::auth_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(web::scope("")
                .wrap(AuthGuard::new())
                .configure(routes::drive_routes::drive_routes)
            )
    })
    .bind(config.serv_addrs)?
    .run()
    .await
}
