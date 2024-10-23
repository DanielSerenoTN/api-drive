use actix_web::{web, HttpResponse, Responder};

use crate::{api::auth::build_auth_url, config::Config, services::auth_service::{AuthCallbackQuery, AuthService}};

#[utoipa::path(
    get,
    path = "/auth/callback",
    params(
        ("code" = String, Query, description = "Authorization code returned by the OAuth2 provider after the user authorizes the application")
    ),
    responses(
        (status = 200, description = "Processes the response from the OAuth2 provider after the redirection, using the authorization code to get an access token."),
        (status = 500, description = "Failed to get access token.")
    ),
    tag = "auth"
)]

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


#[utoipa::path(
    get,
    path = "/auth",
    responses(
        (status = 200, description = "Returns authentication URL")
    ),
    tag = "auth"
)]

pub async fn get_auth_url(
    config: web::Data<Config>
) -> String {
    build_auth_url(config)
}