use actix_web::web;
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;
use crate::config::Config;

#[derive(serde::Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64
}

pub async fn get_access_token(
    code: &str, 
    config: &Config
) -> Result<TokenResponse, Box<dyn Error>> {
    let client: Client = Client::new();

    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("code", code);
    params.insert("client_id", &config.client_id);
    params.insert("client_secret", &config.client_secret);
    params.insert("redirect_uri", &config.redirect_uri);
    params.insert("grant_type", "authorization_code");

    let response = client
        .post(&config.token_uri)
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let token_response = response.json::<TokenResponse>().await?;
        Ok(token_response)
    } else {
        let error_message = response.text().await?;
        Err(Box::from(format!("Error exchanging code for token: {}", error_message)))
    }
}

pub fn build_auth_url(config: web::Data<Config>) -> String {
    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}",
        config.auth_uri, config.client_id, config.redirect_uri, config.scope
    );
    auth_url
}
