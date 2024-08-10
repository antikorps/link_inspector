use crate::app_router::models::{CheckedLink, NonCheckedLink};
use reqwest::Client;

/// Get url status via head request. Async process with futures in chunks of 30
pub async fn verify_links(links: Vec<NonCheckedLink>, client: &Client) -> Vec<CheckedLink> {
    let mut link_index = 0;
    let mut checked_links = Vec::new();
    for chunk in links.chunks(30) {
        let mut futures = Vec::new();
        for link in chunk {
            // Regarding relative links,
            // perhaps the user could be offered a possibility to include the hostname
            // or avoid the request because it is obvious that it will give an error.
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
                        active: 3,
                        relocation: None,
                        error: Some(error_message),
                    });
                    link_index += 1;
                    continue;
                }
                Ok(ok) => response = ok,
            }

            let mut relocation = None;
            if links[link_index].url != response.url().to_string() {
                relocation = Some(response.url().to_string());
            }

            // The active field facilitates front-end management (visualization, grouping of results, order...)
            // 1 (green) means the status is correct (any 2XX)
            // 2 means that a correct status has been received (any 2XX) but the result of a redirection,
            // so perhaps the URL entered is not correct, it must be consulted manually
            // 3 means error, url not available, etc.
            let status = response.status().as_u16();
            let mut active = 3;
            if status < 300 && relocation.is_none() {
                active = 1
            }
            if status < 300 && relocation.is_some() {
                active = 2
            }

            checked_links.push(CheckedLink {
                url: String::from(&links[link_index].url),
                text: String::from(&links[link_index].text),
                status: Some(status),
                active: active,
                relocation,
                error: None,
            });
            link_index += 1;
        }
    }
    checked_links.sort_by_key(|k| k.active);
    return checked_links;
}
