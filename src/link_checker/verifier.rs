use reqwest::Client;

use crate::app_router::models::{CheckedLink, NonCheckedLink};

pub async fn verify_links(links: Vec<NonCheckedLink>, client: &Client) -> Vec<CheckedLink> {
    let mut link_index = 0;
    let mut checked_links = Vec::new();
    for chunk in links.chunks(30) {
        let mut futures = Vec::new();
        for link in chunk {
            futures.push(client.head(&link.url).send());
        }

        let responses = futures::future::join_all(futures).await;

        for r in responses {
            let response;
            match r {
                Err(error) => {
                    let error_message = format!("response error: {error}");
                    checked_links.push(CheckedLink {
                        url: String::from(&links[link_index].url),
                        text: String::from(&links[link_index].text),
                        status: None,
                        active: false,
                        relocation: None,
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
            let mut relocation = None;
            if links[link_index].url != response.url().to_string() {
                relocation = Some(response.url().to_string());
            }
            checked_links.push(CheckedLink {
                url: String::from(&links[link_index].url),
                text: String::from(&links[link_index].text),
                status: Some(status),
                active,
                relocation,
                error: None,
            });
            link_index += 1;
        }
    }
    return checked_links;
}
