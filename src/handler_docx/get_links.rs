use std::collections::HashMap;

use super::process::{HandlerDocx, Link};

impl HandlerDocx {
    pub fn get_links(&mut self) {
        if self.error != None {
            return;
        }
        let mut collection: HashMap<String, usize> = HashMap::new();
        for xml_rel in self.xml_rels.clone() {
            let doc;
            match roxmltree::Document::parse(&xml_rel) {
                Err(error) => {
                    let error_message =
                        format!("could not parse xml on document_xml_rels string: {error}");
                    self.error = Some(error_message);
                    return;
                }
                Ok(ok) => doc = ok,
            }

            let _ = doc.descendants().find(|f| {
                if f.attribute("TargetMode") == None {
                    return false;
                };
                if f.attribute("TargetMode").unwrap() != "External" {
                    return false;
                }
                if f.attribute("Target") == None {
                    return false;
                }
                let link = f.attribute("Target").unwrap().to_string();

                if collection.contains_key(&link) {
                    if let Some(count) = collection.get_mut(&link) {
                        *count += 1;
                    }
                } else {
                    collection.insert(link.to_string(), 1);
                }
                return false;
            });
        }
        let mut links = Vec::new();
        for (key, value) in collection {
            links.push(Link {
                target: key,
                number: value,
            })
        }
        self.links = links;
    }
}
