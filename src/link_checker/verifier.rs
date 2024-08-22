use std::time::Duration;

use crate::app_router::models::{CheckedLink, NonCheckedLink};
use reqwest::Client;
use url::Url;

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

/// Check schema and only verify http, https
pub fn check_schema(link: &NonCheckedLink) -> Option<CheckedLink> {
    let url_parsed = match Url::parse(&link.url) {
        Err(error) => {
            return Some(CheckedLink {
                active: 0,
                url: link.url.clone(),
                text: link.text.clone(),
                status: None,
                error: Some(error.to_string()),
                relocation: None,
            })
        }
        Ok(ok) => ok,
    };
    let valid_schemas = vec!["http", "https"];
    let url_schema = url_parsed.scheme();
    if !valid_schemas.contains(&url_schema) {
        return Some(CheckedLink {
            active: 0,
            url: link.url.clone(),
            text: link.text.clone(),
            status: None,
            error: Some(format!("url schema {} could not be verified", url_schema)),
            relocation: None,
        });
    }
    None
}

pub async fn manage_verify_request(link: &NonCheckedLink, client: &Client) -> CheckedLink {
    match check_schema(link) {
        Some(s) => return s,
        None => (),
    }

    let r;
    match client
        .head(&link.url)
        .timeout(Duration::from_secs(6))
        .send()
        .await
    {
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
        Ok(ok) => {
            if ok.status() == 405 {
                match client
                    .get(&link.url)
                    .timeout(Duration::from_secs(6))
                    .send()
                    .await
                {
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
                    Ok(get_ok) => r = get_ok,
                }
            } else {
                r = ok
            }
        }
    }

    let relocation = check_relocation(&link.url, r.url().as_str());

    // The active field facilitates front-end management (visualization, grouping of results, order...)
    // 1 (green) means the status is correct (any 2XX)
    // 2 means that a correct status has been received (any 2XX) but is the result of a redirection,
    // so perhaps the URL entered is not correct, it must be consulted manually
    // 3 means error, url not available, etc.
    let status = r.status().as_u16();
    let mut active = 3;
    if status > 199 && status < 300 && relocation.is_none() {
        active = 1
    }
    if status > 199 && status < 300 && relocation.is_some() {
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

/// Get url status via head request. Async process with futures in chunks of 100
pub async fn verify_links(links: Vec<NonCheckedLink>, client: &Client) -> Vec<CheckedLink> {
    let mut checked_links = Vec::new();
    for chunk in links.chunks(100) {
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
