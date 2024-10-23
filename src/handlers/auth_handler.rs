use actix_web::{web, HttpResponse, Responder};

use crate::{api::auth::build_auth_url, config::Config, services::auth_service::{AuthCallbackQuery, AuthService}};

pub async fn auth_callback<T: AuthService>(
    query: web::Query<AuthCallbackQuery>,
    config: web::Data<Config>,
    token_service: web::Data<T>,          
) -> impl Responder {
    match token_service.get_access_token(&query.code, &config).await {
        Ok(token_response) => {
            HttpResponse::Ok().body(format!(
                "Access Token: {}\nExpires in: {}",
                token_response.access_token,
                token_response.expires_in
            ))
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error obtaining token: {:?}", err))
        }
    }
}

pub async fn get_auth_url(
    config: web::Data<Config>
) -> String {
    build_auth_url(config)
}