use crate::app_router::models::NonCheckedLink;

use super::process::HandlerHtml;
use scraper::Selector;

impl HandlerHtml {
    pub fn get_links(&mut self) {
        if self.error.is_some() {
            return;
        }
        let selector;
        match Selector::parse("a") {
            Err(error) => {
                let error_message =
                    format!("could not parse a selector, how is it posible ¿? {error}");
                self.error = Some(error_message);
                return;
            }
            Ok(ok) => selector = ok,
        }
        let mut links = Vec::new();
        for element in self.document.as_ref().unwrap().select(&selector) {
            let href = element.attr("href");
            if href.is_none() {
                continue;
            }
            let text = element.text().collect::<String>();
            let url = href.unwrap().to_string();

            links.push(NonCheckedLink { url, text })
        }
        self.links = links;
    }
}
