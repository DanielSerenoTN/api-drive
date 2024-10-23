use actix_web::{web,HttpResponse, Responder};
use serde::Deserialize;
use crate::api::auth::{build_auth_url, get_access_token, TokenResponse};
use crate::config::Config;
use std::future::Future;
use std::error::Error;
use std::pin::Pin;

#[derive(Deserialize)]
pub struct AuthCallbackQuery {
    pub code: String,
}

pub trait AuthService {
    fn get_access_token<'a>(
        &'a self,
        code: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<TokenResponse, Box<dyn Error>>> + 'a>>;
}

pub struct AuthTokenService;

impl AuthService for AuthTokenService {
    fn get_access_token<'a>(
        &'a self,
        code: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<TokenResponse, Box<dyn Error>>> + 'a>> {
        Box::pin(get_access_token(code, config))
    }
}

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

pub async fn get_auth_url(config: web::Data<Config>) -> String {
    build_auth_url(config)
}




