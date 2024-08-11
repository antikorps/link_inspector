use super::{
    models::{CheckedFileType, UploadResponse},
    router::App,
    utilities::content_type_to_extension,
};
use crate::{
    handler_html, handler_office,
    handlers::{pdf_handler::PdfHandler, txt_handler::TxtHandler},
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

        match checked_file_type {
            CheckedFileType::Invalid => {
                let body = UploadResponse {
                    error: Some("file content of file not valid".to_string()),
                    links: None,
                };
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }

            CheckedFileType::Html => match handler_html::process::process_file(file_bytes) {
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
            },

            CheckedFileType::Docx | CheckedFileType::Pptx | CheckedFileType::Xlsx => {
                match handler_office::process::process_file(file_bytes, checked_file_type) {
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
            }
            CheckedFileType::Txt => match TxtHandler::process_file(file_bytes) {
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
            },
            CheckedFileType::Pdf => match PdfHandler::process_file(file_bytes) {
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
            },
        }
    } else {
        let body = UploadResponse {
            error: Some("no file found".to_string()),
            links: None,
        };
        (StatusCode::BAD_REQUEST, Json(body)).into_response()
    }
}
