use crate::{handler_html, handler_office, link_checker};
use axum::{
    body::Bytes,
    extract::{Multipart, State},
    response::Response,
};

use super::{
    models::{CheckedFileType, UploadResponse},
    router::App,
    utilities::content_type_to_extension,
};

/// handle the multipart post request with the file for analysis.
/// Return response UploadResponse: STATUS 200 (the file was processed) or STATUS 400 (file could not be processed)
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

        // Get file bytes
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

    // Could not get file bytes
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

    // Check file type and apply logic corresponding for the file
    let checked_file_type = content_type_to_extension(&file_content_type);
    let links;
    match checked_file_type {
        CheckedFileType::Invalid => {
            let error_message = format!("file content not valid");
            upload_response.error = Some(error_message);
            let upload_response_json = serde_json::to_string(&upload_response)
                .unwrap_or(r###"{"error": "file content not valid", "links": []}"###.to_string());
            return Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(upload_response_json)
                .unwrap();
        }
        CheckedFileType::Html => match handler_html::process::process_file(file_bytes.unwrap()) {
            Err(error) => {
                upload_response.error = Some(error);
                let upload_response_json = serde_json::to_string(&upload_response)
                .unwrap_or(r###"{"error": "critical error proccessing file and could not serialize response", "links": []}"###.to_string());
                return Response::builder()
                    .status(400)
                    .header("Content-Type", "application/json")
                    .body(upload_response_json)
                    .unwrap();
            }
            Ok(ok) => links = ok,
        },
        // To group the 3 Office options all the possibilities must be analyzed beforehand
        // so that the non-exhaustive option allows these 3 possibilities to be grouped.
        _ => match handler_office::process::process_file(file_bytes.unwrap(), checked_file_type) {
            Err(error) => {
                upload_response.error = Some(error);
                let upload_response_json = serde_json::to_string(&upload_response)
                .unwrap_or(r###"{"error": "critical error proccessing file and could not serialize response", "links": []}"###.to_string());
                return Response::builder()
                    .status(400)
                    .header("Content-Type", "application/json")
                    .body(upload_response_json)
                    .unwrap();
            }
            Ok(ok) => links = ok,
        },
    }

    // Verify found links
    let links_checked = link_checker::verifier::verify_links(links, &app.http_client).await;

    upload_response.links = links_checked;
    let upload_response_json = serde_json::to_string(&upload_response).unwrap();
    return Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(upload_response_json)
        .unwrap();
}
