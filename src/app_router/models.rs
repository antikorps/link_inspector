use serde::Serialize;

#[derive(Serialize)]
pub struct UploadResponse {
    pub error: Option<String>,
    pub links: Option<Vec<CheckedLink>>,
}

#[derive(Serialize)]
pub struct CheckedLink {
    pub active: u8,
    pub url: String,
    pub text: String,
    pub status: Option<u16>,
    pub error: Option<String>,
    pub relocation: Option<String>,
}

pub struct NonCheckedLink {
    pub url: String,
    pub text: String,
}
#[derive(PartialEq)]
pub enum CheckedFileType {
    Docx,
    Pptx,
    Xlsx,
    Html,
    Invalid,
}
