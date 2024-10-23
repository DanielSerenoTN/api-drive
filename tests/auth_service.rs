#[path = "mocks/auth_service_mock.rs"]
mod auth_service_mock;
#[path = "mocks/config_mock.rs"]
mod config_mock;

use actix_web::{test, web, App};
use api_drive::handlers::auth_handler::{auth_callback, get_auth_url};
use auth_service_mock::{MockAuthService, MockAuthServiceError};
use config_mock::mock_config;

#[actix_rt::test]
async fn test_auth_callback_success() {
    let config_data = web::Data::new(mock_config());

    let mut app = test::init_service(
        App::new()
            .app_data(config_data.clone())
            .app_data(web::Data::new(MockAuthService))
            .route("/auth/callback", web::get().to(auth_callback::<MockAuthService>)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/auth/callback?code=test_code")
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Access Token: mock_access_token"));
    assert!(body_str.contains("Expires in: 3600"));
}

#[actix_rt::test]
async fn test_auth_callback_error() {
    let config_data = web::Data::new(mock_config());

    let mut app = test::init_service(
        App::new()
            .app_data(config_data.clone())
            .app_data(web::Data::new(MockAuthServiceError))
            .route("/auth/callback", web::get().to(auth_callback::<MockAuthServiceError>)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/auth/callback?code=invalid_code")
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_server_error());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Error obtaining token"));
}

#[actix_rt::test]
async fn test_build_auth_url_happy_path() {
    let config_data = web::Data::new(mock_config());

    let result = get_auth_url(config_data).await;

    assert!(result.contains("client_id=test_client_id"));

    assert!(result.contains("redirect_uri=http://localhost:8080/callback"));
    
    assert!(result.contains("scope=https://www.googleapis.com/auth/drive"));
}


