use std::env;

#[derive(Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: String,
    pub serv_addrs: String,
    pub drive_api_base_url: String,
    pub drive_upload_url: String,
    pub auth_uri: &'static str,
    pub token_uri: &'static str,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let client_id = env::var("CLIENT_ID").expect("Client ID is missing.");
        let client_secret = env::var("CLIENT_SECRET").expect("Client secret is missing.");
        let redirect_uri = env::var("REDIRECT_URI").expect("Redirect URL is missing.");
        let scope = env::var("SCOPE").unwrap_or_else(|_| "https://www.googleapis.com/auth/drive".to_string());
        let serv_addrs = env::var("SERV_ADDRS").expect("Server address is missing.");
        let drive_api_base_url = env::var("GOOGLE_DRIVE_API_BASE_URL").unwrap_or_else(|_| "https://www.googleapis.com/drive/v3/files".to_string());
        let drive_upload_url = env::var("GOOGLE_DRIVE_UPLOAD_URL").unwrap_or_else(|_| "https://www.googleapis.com/upload/drive/v3/files".to_string());
     
        let auth_uri = "https://accounts.google.com/o/oauth2/auth";
        let token_uri = "https://oauth2.googleapis.com/token";

        Config {
            client_id,
            client_secret,
            redirect_uri,
            scope,
            serv_addrs,
            drive_api_base_url,
            drive_upload_url,
            auth_uri,
            token_uri,
        }
    }
}
