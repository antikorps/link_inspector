use std::collections::HashMap;

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
        let mut collection: HashMap<String, usize> = HashMap::new();
        for element in self.document.as_ref().unwrap().select(&selector) {
            let href = element.attr("href");
            if href.is_none() {
                continue;
            }
            let link = href.unwrap().to_string();

            if collection.contains_key(&link) {
                if let Some(count) = collection.get_mut(&link) {
                    *count += 1;
                }
            } else {
                collection.insert(link.to_string(), 1);
            }
        }
        let mut links = Vec::new();
        for (key, value) in collection {
            links.push(NonCheckedLink {
                url: key,
                number: value,
            })
        }
        self.links = links;
    }
}
