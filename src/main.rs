mod config;
mod routes;
mod services;
mod auth;
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
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .supports_credentials()
            )
            .configure(routes::auth_routes::auth_routes)
    })
    .bind(config.serv_addrs)?
    .run()
    .await
}
