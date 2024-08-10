use super::models::CheckedFileType;

/// Check the content type a return a enum for valid or invalid files
pub fn content_type_to_extension(content_type: &str) -> CheckedFileType {
    match content_type {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
            return CheckedFileType::Docx
        }
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
            return CheckedFileType::Pptx
        }
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
            return CheckedFileType::Xlsx
        }
        "text/html" => return CheckedFileType::Html,
        _ => return CheckedFileType::Invalid,
    }
}
