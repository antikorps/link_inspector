use crate::app_router::models::{CheckedLink, NonCheckedLink};
use reqwest::Client;

/// Check a possible relocation of the url.
/// Set the conditions for a flexible relocation
pub fn check_relocation(original_url: &str, final_url: &str) -> Option<String> {
    if original_url == final_url {
        return None;
    }
    // the url https://as.com is relocation because the final url is https://as.com/
    match final_url.strip_suffix("/") {
        None => (),
        Some(ok) => {
            if original_url == ok {
                return None;
            }
        }
    }
    return Some(final_url.to_string());
}

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

            let relocation = check_relocation(&links[link_index].url, response.url().as_str());

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
