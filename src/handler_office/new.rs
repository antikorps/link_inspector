use axum::body::Bytes;

use super::process::HandlerOffice;
use crate::app_router::models::CheckedFileType;

pub fn new_handler_office(file_bytes: Bytes, checked_file_type: CheckedFileType) -> HandlerOffice {
    return HandlerOffice {
        file_bytes,
        checked_file_type,
        xml_rels: Vec::new(),
        error: None,
        links: Vec::new(),
    };
}
