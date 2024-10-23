mod config;
mod routes;
mod services;
mod api;
mod handlers;
use config::Config;
use actix_web::{web,App, HttpServer};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    let config_data = web::Data::new(config.clone());

    println!("Starting server at {}", config.serv_addrs);
    
    HttpServer::new(move || {
        App::new()
            .app_data(config_data.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header()
            )
            .configure(routes::auth_routes::auth_routes)
            .configure(routes::drive_routes::drive_routes)
    })
    .bind(config.serv_addrs)?
    .run()
    .await
}
