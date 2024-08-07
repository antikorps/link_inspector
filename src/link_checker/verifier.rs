use reqwest::Client;
use serde::Serialize;

use crate::handler_docx::process::Link;

#[derive(Serialize)]
pub struct CheckedLink {
    pub url: String,
    pub status: Option<u16>,
    pub active: bool,
    pub number: usize,
    pub error: Option<String>,
}

pub async fn verify_links(links: Vec<Link>, client: &Client) -> Vec<CheckedLink> {
    let mut link_index = 0;
    let mut checked_links = Vec::new();
    for chunk in links.chunks(10) {
        let mut futures = Vec::new();
        for link in chunk {
            futures.push(client.head(&link.target).send());
        }

        let responses = futures::future::join_all(futures).await;

        for r in responses {
            let response;
            match r {
                Err(error) => {
                    let error_message = format!("response error: {error}");
                    checked_links.push(CheckedLink {
                        url: String::from(&links[link_index].target),
                        status: None,
                        active: false,
                        number: links[link_index].number,
                        error: Some(error_message),
                    });
                    link_index += 1;
                    continue;
                }
                Ok(ok) => response = ok,
            }
            let mut active = false;
            let status = response.status().as_u16();
            if status < 300 {
                active = true;
            };
            checked_links.push(CheckedLink {
                url: String::from(&links[link_index].target),
                status: Some(status),
                active,
                number: links[link_index].number,
                error: None,
            });
            link_index += 1;
        }
    }
    return checked_links;
}
