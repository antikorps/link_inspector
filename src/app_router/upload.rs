use axum::{
    body::Bytes,
    extract::{Multipart, State},
    response::Response,
};
use serde::Serialize;

use crate::{
    handler_docx,
    link_checker::{self, verifier::CheckedLink},
};

use super::router::App;

#[derive(Serialize)]
struct UploadResponse {
    error: Option<String>,
    links: Vec<CheckedLink>,
}

pub async fn upload(State(app): State<App>, mut multipart: Multipart) -> Response<String> {
    let mut file_bytes: Option<Bytes> = None;
    let mut file_content_type = String::new();
    let mut upload_response = UploadResponse {
        error: None,
        links: Vec::new(),
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.content_type() == None || field.name() == None {
            continue;
        }

        let field_name = field.name().unwrap().to_string();
        if field_name != "file" {
            continue;
        }
        file_content_type = field.content_type().unwrap().to_string();

        match field.bytes().await {
            Err(error) => {
                let error_message = format!("could not get file bytes {error}");
                upload_response.error = Some(error_message);
                let upload_response_json = serde_json::to_string(&upload_response).unwrap();
                return Response::builder()
                    .status(400)
                    .header("Content-Type", "application/json")
                    .body(upload_response_json)
                    .unwrap();
            }
            Ok(ok) => {
                file_bytes = Some(ok);
            }
        }
    }

    if file_bytes == None {
        let error_message = format!("could not get file bytes for the field file");
        upload_response.error = Some(error_message);
        let upload_response_json = serde_json::to_string(&upload_response).unwrap();
        return Response::builder()
            .status(400)
            .header("Content-Type", "application/json")
            .body(upload_response_json)
            .unwrap();
    }

    if file_content_type
        == "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    {
        // HANDLER DOCX
        let links = handler_docx::process::process_file(file_bytes.unwrap());
        let links_checked = link_checker::verifier::verify_links(links, &app.http_client).await;

        upload_response.links = links_checked;
        let upload_response_json = serde_json::to_string(&upload_response).unwrap();
        return Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(upload_response_json)
            .unwrap();
    }

    let error_message = format!("file content of file not valid");
    upload_response.error = Some(error_message);
    let upload_response_json = serde_json::to_string(&upload_response).unwrap();
    return Response::builder()
        .status(400)
        .header("Content-Type", "application/json")
        .body(upload_response_json)
        .unwrap();
}