use serde::Serialize;

#[derive(Serialize)]
pub struct CheckedLink {
    pub active: u8,
    pub url: String,
    pub text: String,
    pub status: Option<u16>,
    pub error: Option<String>,
    pub relocation: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct NonCheckedLink {
    pub url: String,
    pub text: String,
}
#[derive(PartialEq)]
pub enum FileType {
    Docx,
    Pptx,
    Xlsx,
    Html,
    Txt,
    Pdf,
    Invalid,
}
