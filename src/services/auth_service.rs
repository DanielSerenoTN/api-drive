use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use crate::api::auth::{get_access_token, TokenResponse};
use crate::config::Config;
use std::future::Future;
use std::pin::Pin;
use anyhow::{Context, Result};

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct AuthCallbackQuery {
    pub code: String,
}

pub trait AuthService {
    fn get_access_token<'a>(
        &'a self,
        code: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<TokenResponse>> + Send + 'a>>;
}

pub struct AuthTokenService;

impl AuthService for AuthTokenService {
    fn get_access_token<'a>(
        &'a self,
        code: &'a str,
        config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<TokenResponse>> + Send + 'a>> {
        Box::pin(async move {
            get_access_token(code, config)
                .await
                .context("Failed to get access token")
        })
    }
}
