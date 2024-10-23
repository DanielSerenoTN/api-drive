mod config;
mod routes;
mod services;
mod api;
mod handlers;
mod middlewares;
mod swagger_config;
use config::Config;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use middlewares::auth_guard::AuthGuard;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


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
                    .url("/api-docs/openapi.json", swagger_config::ApiDoc::openapi())
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
