use api_drive::config::Config;
use api_drive::api::auth::TokenResponse;
use api_drive::services::auth_service::AuthService;
use std::future::Future;
use std::pin::Pin;
use std::error::Error;

pub struct MockAuthService;

impl AuthService for MockAuthService {
    fn get_access_token<'a>(
        &'a self,
        _code: &'a str,
        _config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<TokenResponse, Box<dyn Error>>> + 'a>> {
        Box::pin(async {
            Ok(TokenResponse {
                access_token: "mock_access_token".to_string(),
                expires_in: 3600,
            })
        })
    }
}

pub struct MockAuthServiceError;

impl AuthService for MockAuthServiceError {
    fn get_access_token<'a>(
        &'a self,
        _code: &'a str,
        _config: &'a Config
    ) -> Pin<Box<dyn Future<Output = Result<TokenResponse, Box<dyn Error>>> + 'a>> {
        Box::pin(async {
            Err(Box::from("Mock error exchanging code for token"))
        })
    }
}
