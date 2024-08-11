extern crate linkify;

use axum::body::Bytes;
use linkify::{LinkFinder, LinkKind};
use url::Url;

use crate::app_router::models::NonCheckedLink;

pub struct TxtHandler {}

impl TxtHandler {
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

    pub fn process_file(file_bytes: Bytes) -> Result<Vec<NonCheckedLink>, String> {
        match Self::parse_file(file_bytes) {
            Ok(content) => {
                let links = Self::get_links(content);
                Ok(links)
            }

            Err(error) => Err(error),
        }
    }

    fn parse_file(file_bytes: Bytes) -> Result<String, String> {
        match String::from_utf8(file_bytes.to_vec()) {
            Err(_error) => {
                let error_message = "error reading the file";
                Err(error_message.to_string())
            }

            Ok(content) => Ok(content),
        }
    }
}
