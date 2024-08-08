use axum::body::Bytes;

use super::{parser::html_parser_document, process::HandlerHtml};

pub fn new_handler_html(file_bytes: Bytes) -> HandlerHtml {
    let mut handler_html = HandlerHtml {
        document: None,
        error: None,
        links: Vec::new(),
    };
    match String::from_utf8(file_bytes.to_vec()) {
        Err(error) => {
            let error_message = format!("could not get the content for the html file {error}");
            handler_html.error = Some(error_message);
            return handler_html;
        }
        Ok(ok) => {
            handler_html.document = Some(html_parser_document(ok));
            return handler_html;
        }
    }
}
