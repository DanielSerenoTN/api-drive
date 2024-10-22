use crate::{config::Config, services::auth_service::{auth_callback, get_auth_url, AuthTokenService}};
use actix_web::web;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .app_data(web::Data::new(AuthTokenService))
            .route("", web::get().to(|config: web::Data<Config>, _token_service: web::Data<AuthTokenService>| get_auth_url(config)))
            .route("/callback", web::get().to(auth_callback::<AuthTokenService>))
    );
}
