use axum::body::Bytes;
use scraper::Html;

use crate::app_router::models::NonCheckedLink;

use super::new::new_handler_html;

pub struct HandlerHtml {
    pub document: Option<Html>,
    pub error: Option<String>,
    pub links: Vec<NonCheckedLink>,
}

pub fn process_file(file_bytes: Bytes) -> Result<Vec<NonCheckedLink>, String> {
    let mut handler = new_handler_html(file_bytes);
    handler.get_links();
    if handler.error.is_some() {
        return Err(handler.error.unwrap());
    }
    return Ok(handler.links);
}
