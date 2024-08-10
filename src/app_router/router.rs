use crate::http_client;
use axum::{extract::DefaultBodyLimit, routing::post, Router};
use axum_embed::ServeEmbed;
use reqwest::Client;
use rust_embed::RustEmbed;

use super::upload::upload;

#[derive(RustEmbed, Clone)]
#[folder = "frontend_alt/dist"]
struct StaticFiles;

#[derive(Clone)]
pub struct App {
    pub http_client: Client,
}

/// Create route for serve static files and upload
pub async fn make_router() -> Router {
    let serve_static = ServeEmbed::<StaticFiles>::new();
    let app = App {
        http_client: http_client::create_client::create().await,
    };

    // /upload has a limit size of 32MG for avoid attacks.
    // If the app if self hosted can be increase
    return Router::new()
        .route("/upload", post(upload))
        .layer(DefaultBodyLimit::max(32000 * 1024))
        .nest_service("/", serve_static)
        .with_state(app);
}
