use super::models::CheckedLink;
use serde::Serialize;

#[derive(Serialize)]
pub struct UploadResponse {
    pub links: Vec<CheckedLink>,
}
