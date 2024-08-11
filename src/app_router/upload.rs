use super::{
    models::{CheckedFileType, UploadResponse},
    router::App,
    utilities::content_type_to_extension,
};
use crate::{
    handlers::{
        html_handler::HtmlHandler, office_handler::OfficeHandler, pdf_handler::PdfHandler,
        txt_handler::TxtHandler,
    },
    link_checker::verifier::verify_links,
};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub async fn upload(State(app): State<App>, mut multipart: Multipart) -> Response {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let file_content_type = field.content_type().unwrap().to_string();
        let file_bytes = field.bytes().await.unwrap();
        let checked_file_type = content_type_to_extension(&file_content_type);

        let result = match checked_file_type {
            CheckedFileType::Html => HtmlHandler::process_file(file_bytes),
            CheckedFileType::Docx | CheckedFileType::Pptx | CheckedFileType::Xlsx => {
                OfficeHandler::process_file(file_bytes, checked_file_type)
            }
            CheckedFileType::Txt => TxtHandler::process_file(file_bytes),
            CheckedFileType::Pdf => PdfHandler::process_file(file_bytes),
            CheckedFileType::Invalid => {
                let body = UploadResponse {
                    error: Some("file content of file not valid".to_string()),
                    links: None,
                };
                return (StatusCode::BAD_REQUEST, Json(body)).into_response();
            }
        };

        match result {
            Err(error) => {
                let body = UploadResponse {
                    error: Some(error),
                    links: None,
                };
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
            Ok(unchecked_links) => {
                let body = UploadResponse {
                    error: None,
                    links: Some(verify_links(unchecked_links, &app.http_client).await),
                };
                (StatusCode::OK, Json(body)).into_response()
            }
        }
    } else {
        let body = UploadResponse {
            error: Some("no file found".to_string()),
            links: None,
        };
        (StatusCode::BAD_REQUEST, Json(body)).into_response()
    }
}
