extern crate linkify;

use axum::body::Bytes;
use linkify::{LinkFinder, LinkKind};
use lopdf::Document;
use url::Url;

use crate::app_router::models::NonCheckedLink;

pub struct PdfHandler {}

impl PdfHandler {
    fn get_links(text: String) -> Vec<NonCheckedLink> {
        let mut finder = LinkFinder::new();
        finder.kinds(&[LinkKind::Url]);
        finder
            .links(&text)
            .map(|link| NonCheckedLink {
                url: link.as_str().to_string(),
                text: match Url::parse(link.as_str()) {
                    Ok(url) => url.host_str().unwrap().to_string(),
                    Err(_) => link.as_str().to_string(),
                },
            })
            .collect()
    }

    pub fn process_file(
        file_bytes: Bytes
    ) -> Result<Vec<NonCheckedLink>, String> {
        match Self::parse_file(file_bytes) {
            Ok(content) => {
                let links = Self::get_links(content);
                Ok(links)
            }

            Err(error) => Err(error),
        }
    }

    fn parse_file(file_bytes: Bytes) -> Result<String, String> {
        match Document::load_mem(file_bytes.to_vec().as_slice()) {
            Ok(document) => {
                let mut annotations: Vec<String> = Vec::new();
                for (_, object_id) in document.get_pages().iter() {
                    let page_annotations = document.get_page_annotations(*object_id);
                    page_annotations.iter().for_each(|annotation| {
                        annotations.push(format!("{:?}", annotation));
                    });
                }
                Ok(annotations.join("\n").to_string())
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
