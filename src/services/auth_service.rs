use serde::Deserialize;
use crate::api::auth::{get_access_token, TokenResponse};
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








