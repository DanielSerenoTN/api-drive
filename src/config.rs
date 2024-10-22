use std::env;

#[derive(Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub scope: String,
    pub serv_addrs: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let client_id = env::var("CLIENT_ID").expect("Client ID is missing.");
        let client_secret = env::var("CLIENT_SECRET").expect("Client secret is missing.");
        let redirect_uri = env::var("REDIRECT_URI").expect("Redirect URL is missing.");
        let auth_uri = env::var("AUTH_URI").expect("Auth URL is missing.");
        let token_uri = env::var("TOKEN_URI").expect("Token URL is missing.");
        let scope = env::var("SCOPE").expect("Scope is missing.");
        let serv_addrs = env::var("SERV_ADDRS").expect("Server address is missing.");
        
        Config {
            client_id,
            client_secret,
            redirect_uri,
            auth_uri,
            token_uri,
            scope,
            serv_addrs,
        }
    }
}