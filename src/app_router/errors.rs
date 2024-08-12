use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct ErrorResponseBody {
    error: String,
}

#[derive(Error, Debug)]
pub enum UploadFileError {
    #[error("Archivo no encontrado")]
    FileNotFound(),

    #[error("Tipo de fichero no válido: {0}")]
    InvalidFileType(String),
}

impl IntoResponse for UploadFileError {
    fn into_response(self) -> Response {
        let status = match self {
            UploadFileError::FileNotFound() => StatusCode::NOT_FOUND,
            UploadFileError::InvalidFileType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        };

        let body = ErrorResponseBody {
            error: self.to_string(),
        };

        (status, Json(body)).into_response()
    }
}
