use std::time::Duration;

use reqwest::{header, Client};

pub async fn create() -> Client {
    let mut cabeceras = header::HeaderMap::new();
    cabeceras.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str(
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0",
        )
        .unwrap(),
    );

    reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(6))
        .default_headers(cabeceras)
        .build()
        .expect("ERROR FATAL: no se ha podido generar el cliente http")
}
