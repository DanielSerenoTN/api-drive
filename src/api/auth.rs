use actix_web::web;
use reqwest::Client;
use std::collections::HashMap;
use anyhow::{Result, Context};
use crate::config::Config;

#[derive(serde::Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64
}

pub async fn get_access_token(
    code: &str, 
    config: &Config
) -> Result<TokenResponse> {
    let client: Client = Client::new();

    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("code", code);
    params.insert("client_id", &config.client_id);
    params.insert("client_secret", &config.client_secret);
    params.insert("redirect_uri", &config.redirect_uri);
    params.insert("grant_type", "authorization_code");

    let response = client
        .post(config.token_uri)
        .form(&params)
        .send()
        .await
        .context("Failed to send request to token URI")?;

    if response.status().is_success() {
        let token_response = response
            .json::<TokenResponse>()
            .await
            .context("Failed to parse token response")?;
        Ok(token_response)
    } else {
        let error_message = response
            .text()
            .await
            .context("Failed to retrieve error message from response")?;
        anyhow::bail!("Error exchanging code for token: {}", error_message);
    }
}

pub fn build_auth_url(config: web::Data<Config>) -> String {
    format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}",
        config.auth_uri, config.client_id, config.redirect_uri, config.scope
    )
}

pub async fn validate_token(token: String) -> Result<bool> {
    let client = Client::new();
    
    let url = format!("https://www.googleapis.com/oauth2/v3/tokeninfo?access_token={}", token);

    let response = client.get(&url).send().await.context("Failed to validate token")?;

    if response.status().is_success() {
        let json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse token validation response")?;
        println!("Token info: {:?}", json);
        Ok(true)
    } else {
        println!("Token is invalid or expired.");
        Ok(false)
    }
}
