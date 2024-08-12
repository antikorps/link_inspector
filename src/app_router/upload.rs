use super::{models::FileType, responses::UploadResponse, router::App};
use crate::{
    app_router::errors::UploadFileError,
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
        let type_extension = content_type_to_extension(&file_content_type);

        let result = match type_extension {
            FileType::Html => HtmlHandler::process_file(file_bytes),
            FileType::Docx | FileType::Pptx | FileType::Xlsx => {
                OfficeHandler::process_file(file_bytes, type_extension)
            }
            FileType::Txt => TxtHandler::process_file(file_bytes),
            FileType::Pdf => PdfHandler::process_file(file_bytes),
            FileType::Invalid => {
                return UploadFileError::InvalidFileType(file_content_type).into_response()
            }
        };

        match result {
            Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
            Ok(unchecked_links) => {
                let body = UploadResponse {
                    links: verify_links(unchecked_links, &app.http_client).await,
                };
                (StatusCode::OK, Json(body)).into_response()
            }
        }
    } else {
        UploadFileError::FileNotFound().into_response()
    }
}

pub fn content_type_to_extension(content_type: &str) -> FileType {
    match content_type {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => FileType::Docx,
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
            FileType::Pptx
        }
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => FileType::Xlsx,
        "text/plain" => FileType::Txt,
        "text/html" => FileType::Html,
        "application/pdf" => FileType::Pdf,
        _ => FileType::Invalid,
    }
}
