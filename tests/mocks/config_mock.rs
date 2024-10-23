use api_drive::config::Config;

pub fn mock_config() -> Config {
    Config {
        client_id: "test_client_id".to_string(),
        client_secret: "test_secret".to_string(),
        redirect_uri: "http://localhost:8080/callback".to_string(),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        token_uri: "https://oauth2.googleapis.com/token".to_string(),
        scope: "https://www.googleapis.com/auth/drive".to_string(),
        serv_addrs: "127.0.0.1:8080".to_string(),
        drive_api_base_url: "https://www.googleapis.com/drive/v3/files".to_string(),
    }
}
