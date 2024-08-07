use axum::body::Bytes;

use super::process::HandlerDocx;

pub fn new_handler_docx(file_bytes: Bytes) -> HandlerDocx {
    return HandlerDocx {
        file_bytes,
        xml_rels: Vec::new(),
        error: None,
        links: Vec::new(),
    };
}
