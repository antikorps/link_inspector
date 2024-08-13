use crate::app_router::models::{CheckedLink, NonCheckedLink};
use reqwest::Client;
use url::{ParseError, Url};

/// Check a possible relocation of the url.
/// Set the conditions for a flexible relocation
pub fn check_relocation(original_url: &str, final_url: &str) -> Option<String> {
    if original_url == final_url {
        return None;
    }
    // the url https://as.com is relocation because the final url is https://as.com/
    match final_url.strip_suffix('/') {
        None => (),
        Some(ok) => {
            if original_url == ok {
                return None;
            }
        }
    }
    Some(final_url.to_string())
}

pub async fn manage_verify_request(link: &NonCheckedLink, client: &Client) -> CheckedLink {
    if link.url.starts_with('/') {
        return CheckedLink {
            active: 3,
            url: link.url.clone(),
            text: link.text.clone(),
            status: None,
            error: Some("url could not start with /, probably it is a relative url".to_string()),
            relocation: None,
        };
    }
    let url_endpoint = match Url::parse(&link.url) {
        Err(error) => {
            if error == ParseError::RelativeUrlWithoutBase {
                format!("https://{}", link.url)
            } else {
                return CheckedLink {
                    active: 3,
                    url: link.url.clone(),
                    text: link.text.clone(),
                    status: None,
                    error: Some(error.to_string()),
                    relocation: None,
                };
            }
        }
        Ok(ok) => ok.to_string(),
    };

    let r;
    match client.get(url_endpoint).send().await {
        Err(error) => {
            return CheckedLink {
                active: 3,
                url: link.url.clone(),
                text: link.text.clone(),
                status: None,
                error: Some(error.to_string()),
                relocation: None,
            }
        }
        Ok(ok) => r = ok,
    }

    let relocation = check_relocation(&link.url, r.url().as_str());

    // The active field facilitates front-end management (visualization, grouping of results, order...)
    // 1 (green) means the status is correct (any 2XX)
    // 2 means that a correct status has been received (any 2XX) but the result of a redirection,
    // so perhaps the URL entered is not correct, it must be consulted manually
    // 3 means error, url not available, etc.
    let status = r.status().as_u16();
    let mut active = 3;
    if status < 300 && relocation.is_none() {
        active = 1
    }
    if status < 300 && relocation.is_some() {
        active = 2
    }

    CheckedLink {
        active,
        url: link.url.clone(),
        text: link.text.clone(),
        status: Some(status),
        error: None,
        relocation,
    }
}

/// Get url status via head request. Async process with futures in chunks of 30
pub async fn verify_links(links: Vec<NonCheckedLink>, client: &Client) -> Vec<CheckedLink> {
    let mut checked_links = Vec::new();
    for chunk in links.chunks(30) {
        let mut futures = Vec::new();
        for link in chunk {
            futures.push(manage_verify_request(link, client));
        }
        let responses = futures::future::join_all(futures).await;
        for r in responses {
            checked_links.push(r)
        }
    }
    checked_links.sort_by_key(|k| k.active);
    checked_links
}
