use axum::body::Bytes;
use scraper::{Html, Selector};

use crate::app_router::models::NonCheckedLink;

pub struct HtmlHandler {}

impl HtmlHandler {
    pub fn get_links(content: String) -> Result<Vec<NonCheckedLink>, String> {
        let parsed_html = Html::parse_document(&content);

        match Selector::parse("a") {
            Err(error) => {
                let error_message =
                    format!("could not parse a selector, how is it posible ¿? {error}");
                return Err(error_message);
            }
            Ok(selector) => {
                let mut links = Vec::new();
                for element in parsed_html.select(&selector) {
                    let href = element.attr("href");
                    if href.is_none() {
                        continue;
                    }
                    let text = element.text().collect::<String>();
                    let url = href.unwrap().to_string();

                    links.push(NonCheckedLink { url, text })
                }
                return Ok(links);
            }
        }
    }

    pub fn process_file(
        file_bytes: Bytes
    ) -> Result<Vec<NonCheckedLink>, String> {
        match String::from_utf8(file_bytes.to_vec()) {
            Err(error) => {
                let error_message = format!("could not get the content for the html file {error}");
                return Err(error_message);
            }
            Ok(ok) => {
                return Self::get_links(ok);
            }
        }
    }
}
