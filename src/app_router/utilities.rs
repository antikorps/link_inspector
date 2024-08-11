use super::models::CheckedFileType;

/// Check the content type a return a enum for valid or invalid files
pub fn content_type_to_extension(content_type: &str) -> CheckedFileType {
    match content_type {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
            CheckedFileType::Docx
        }
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
            CheckedFileType::Pptx
        }
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
            CheckedFileType::Xlsx
        }
        "text/plain" => CheckedFileType::Txt,
        "application/pdf" => CheckedFileType::Pdf,
        "text/html" => CheckedFileType::Html,
        _ => CheckedFileType::Invalid,
    }
}
