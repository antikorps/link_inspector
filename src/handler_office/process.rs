use axum::body::Bytes;

use crate::app_router::models::NonCheckedLink;

use super::new::new_handler_office;
use crate::app_router::models::CheckedFileType;

pub struct HandlerOffice {
    pub file_bytes: Bytes,
    pub checked_file_type: CheckedFileType,
    pub xml_rels: Vec<String>,
    pub error: Option<String>,
    pub links: Vec<NonCheckedLink>,
}

pub fn process_file(
    file_bytes: Bytes,
    checked_file_type: CheckedFileType,
) -> Result<Vec<NonCheckedLink>, String> {
    let mut handler = new_handler_office(file_bytes, checked_file_type);
    handler.get_xml_rels();
    handler.get_links();
    if handler.error.is_some() {
        return Err(handler.error.unwrap());
    }
    return Ok(handler.links);
}
