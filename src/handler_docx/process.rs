use axum::body::Bytes;

use super::new::new_handler_docx;

pub struct HandlerDocx {
    pub file_bytes: Bytes,
    pub xml_rels: Vec<String>,
    pub error: Option<String>,
    pub links: Vec<Link>,
}

pub struct Link {
    pub target: String,
    pub number: usize,
}

pub fn process_file(file_bytes: Bytes) -> Vec<Link> {
    let mut handler = new_handler_docx(file_bytes);
    handler.get_xml_rels();
    handler.get_links();
    return handler.links;
}
