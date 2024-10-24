use actix_web::{test, web, App, http::header};
use api_drive::handlers::google_drive_handler::{download_pdf_file_by_id, get_list_folders};
use api_drive::services::google_drive_service::FolderInfo;

#[path = "mocks/google_drive_service_mock.rs"]
mod google_drive_service_mock;

#[path = "mocks/config_mock.rs"]
mod config_mock;

use google_drive_service_mock::MockGoogleDriveService;
use config_mock::mock_config;

#[actix_web::test]
async fn test_get_list_folders_success() {

    let mock_service = web::Data::new(MockGoogleDriveService);

    let config_data = web::Data::new(mock_config());

    let app = test::init_service(
        App::new()
            .app_data(mock_service.clone())
            .app_data(config_data.clone())
            .route("/drive/list-folders", web::get().to(get_list_folders::<MockGoogleDriveService>)),
    ).await;

    let req = test::TestRequest::get()
        .uri("/drive/list-folders")
        .insert_header((header::AUTHORIZATION, "Bearer mock_token"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success(), "Response was not successful");

    let result: Vec<FolderInfo> = test::read_body_json(resp).await;

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].name, Some("Folder 1".to_string()));
    assert_eq!(result[1].name, Some("Folder 2".to_string()));
}

#[actix_web::test]
async fn test_get_list_folders_unauthorized() {

    let mock_service: web::Data<MockGoogleDriveService> = web::Data::new(MockGoogleDriveService);

    let config_data = web::Data::new(mock_config());

    let app = test::init_service(
        App::new()
            .app_data(mock_service.clone())
            .app_data(config_data.clone())
            .route("/drive/list-folders", web::get().to(get_list_folders::<MockGoogleDriveService>)),
    ).await;

    let req = test::TestRequest::get()
        .uri("/drive/list-folders")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400, "Expected 400 status for missing token");

    let body = test::read_body(resp).await;

    assert_eq!(body, web::Bytes::from_static(b"Authorization token missing or invalid"));
}

#[actix_web::test]
async fn test_download_pdf_file_by_id_success() {
    let mock_service = web::Data::new(MockGoogleDriveService);
    let config_data = web::Data::new(mock_config());

    let app = test::init_service(
        App::new()
            .app_data(mock_service.clone())
            .app_data(config_data.clone())
            .route("/drive/files/{file_id}", web::get().to(download_pdf_file_by_id::<MockGoogleDriveService>)),
    )
    .await;

    let file_id = "test_file_id";

    let req = test::TestRequest::get()
        .uri(&format!("/drive/files/{}", file_id))
        .insert_header((header::AUTHORIZATION, "Bearer mock_token"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success(), "Response was not successful");

    let body = test::read_body(resp).await;
    assert_eq!(body, web::Bytes::from_static(&[0x25, 0x50, 0x44, 0x46]));
}

#[actix_web::test]
async fn test_download_pdf_file_by_id_unauthorized() {
    let mock_service = web::Data::new(MockGoogleDriveService);
    let config_data = web::Data::new(mock_config());

    let app = test::init_service(
        App::new()
            .app_data(mock_service.clone())
            .app_data(config_data.clone())
            .route("/drive/files/{file_id}", web::get().to(download_pdf_file_by_id::<MockGoogleDriveService>)),
    )
    .await;

    let file_id = "test_file_id";

    let req = test::TestRequest::get()
        .uri(&format!("/drive/files/{}", file_id))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400, "Expected 400 status for missing token");

    let body = test::read_body(resp).await;
    assert_eq!(body, web::Bytes::from_static(b"Authorization token missing or invalid"));
}