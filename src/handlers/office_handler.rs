use std::io::{Cursor, Read};

use axum::body::Bytes;
use roxmltree::Document;
use zip::ZipArchive;

use crate::app_router::models::{FileType, NonCheckedLink};

struct OfficeRelationLinkText {
    id: String,
    text: String,
    link: String,
    content_text_file: String,
}

pub struct OfficeHandler {}

impl OfficeHandler {
    pub fn process_file(
        file_bytes: Bytes,
        checked_file_type: FileType,
    ) -> Result<Vec<NonCheckedLink>, String> {
        let cursor = Cursor::new(file_bytes);
        let zip = match ZipArchive::new(cursor) {
            Err(error) => {
                let error_message = format!("could not read office file as a zip: {error}");
                return Err(error_message);
            }
            Ok(ok) => ok,
        };
        Self::get_links(zip, checked_file_type)
    }

    fn get_links(
        mut zip: ZipArchive<Cursor<Bytes>>,
        checked_file_type: FileType,
    ) -> Result<Vec<NonCheckedLink>, String> {
        let relations_link_text = Self::extract_relationships(&mut zip, checked_file_type)?;
        let mut non_checked_links = Vec::new();

        for relation in relations_link_text {
            non_checked_links.push(NonCheckedLink {
                url: relation.link,
                text: relation.text,
            });
        }

        Ok(non_checked_links)
    }

    fn extract_relationships(
        zip: &mut ZipArchive<Cursor<Bytes>>,
        checked_file_type: FileType,
    ) -> Result<Vec<OfficeRelationLinkText>, String> {
        let mut relations_link_text = Vec::new();
        let mut content_text_files = Vec::new();

        for i in 0..zip.len() {
            let mut file = match zip.by_index(i) {
                Err(_) => continue,
                Ok(file) => file,
            };

            if !Self::is_valid_file(&file.name(), &checked_file_type) {
                continue;
            }

            let xml_rels = Self::read_to_string(&mut file)?;
            let doc = Self::parse_xml(&xml_rels)?;

            for node in doc
                .descendants()
                .filter(|f| f.attribute("TargetMode") == Some("External"))
            {
                if let (Some(id), Some(link)) = (node.attribute("Id"), node.attribute("Target")) {
                    let content_text_file = file
                        .name()
                        .to_string()
                        .strip_suffix(".rels")
                        .unwrap()
                        .to_string();
                    content_text_files.push(content_text_file.clone());
                    relations_link_text.push(OfficeRelationLinkText {
                        content_text_file,
                        id: id.to_string(),
                        text: String::new(),
                        link: link.to_string(),
                    });
                }
            }
        }

        Self::associate_texts_with_relations(zip, &checked_file_type, &mut relations_link_text)?;
        Ok(relations_link_text)
    }

    fn is_valid_file(file_name: &str, checked_file_type: &FileType) -> bool {
        match checked_file_type {
            FileType::Docx => file_name.starts_with("word/_rels/"),
            FileType::Pptx => file_name.starts_with("ppt/slides/_rels"),
            FileType::Xlsx => file_name.starts_with("xl/worksheets/_rels"),
            _ => false,
        }
    }

    fn read_to_string(file: &mut dyn Read) -> Result<String, String> {
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(error) => {
                let error_message = format!("could not read file content: {error}");
                return Err(error_message);
            }
            Ok(_) => (),
        };
        Ok(content)
    }

    fn parse_xml(content: &str) -> Result<Document, String> {
        Document::parse(content).map_err(|e| format!("could not parse XML: {e}"))
    }

    fn associate_texts_with_relations(
        zip: &mut ZipArchive<Cursor<Bytes>>,
        checked_file_type: &FileType,
        relations_link_text: &mut Vec<OfficeRelationLinkText>,
    ) -> Result<(), String> {
        for relation in relations_link_text.iter_mut() {
            let content_file_path =
                Self::get_content_file_path(&relation.content_text_file, &checked_file_type);
            let mut content_file = match zip.by_name(&content_file_path) {
                Err(error) => {
                    let error_message = format!("file not found: {content_file_path} {error}");
                    return Err(error_message);
                }
                Ok(ok) => ok,
            };

            let content_file_xml = Self::read_to_string(&mut content_file)?;
            let doc = Self::parse_xml(&content_file_xml)?;

            for node in doc.descendants() {
                Self::update_relation_text(relation, &node, &checked_file_type);
            }
        }
        Ok(())
    }

    fn get_content_file_path(content_text_file: &str, checked_file_type: &FileType) -> String {
        match checked_file_type {
            FileType::Docx => format!(
                "word/{}",
                content_text_file.strip_prefix("word/_rels/").unwrap()
            ),
            FileType::Pptx => format!(
                "ppt/slides/{}",
                content_text_file.strip_prefix("ppt/slides/_rels/").unwrap()
            ),
            FileType::Xlsx => format!(
                "xl/worksheets/{}",
                content_text_file
                    .strip_prefix("xl/worksheets/_rels/")
                    .unwrap()
            ),
            _ => "".to_string(), // It can not be possible
        }
    }

    fn update_relation_text(
        relation: &mut OfficeRelationLinkText,
        node: &roxmltree::Node,
        checked_file_type: &FileType,
    ) {
        match checked_file_type {
            FileType::Docx => {
                if node.has_tag_name((
                    "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                    "hyperlink",
                )) {
                    Self::extract_text_and_update_relation(relation, node, "t");
                }
            }
            FileType::Pptx => {
                if node.has_tag_name(("http://schemas.openxmlformats.org/drawingml/2006/main", "r"))
                {
                    Self::extract_text_and_update_relation(relation, node, "t");
                }
            }
            FileType::Xlsx => {
                if node.has_tag_name((
                    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                    "hyperlink",
                )) {
                    if let Some(id) = node.attribute((
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                        "id",
                    )) {
                        if relation.id == id {
                            if let Some(display) = node.attribute("display") {
                                if let Some(cell_ref) = node.attribute("ref") {
                                    relation.text = format!("{} ({})", display, cell_ref);
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn extract_text_and_update_relation(
        relation: &mut OfficeRelationLinkText,
        node: &roxmltree::Node,
        tag_name: &str,
    ) {
        let mut node_id = String::new();
        let mut node_text = String::new();

        node.descendants().for_each(|descendant| {
            if let Some(id) = descendant.attribute((
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                "id",
            )) {
                node_id = id.to_string();
            }
            if descendant.has_tag_name((
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                tag_name,
            )) {
                if let Some(text) = descendant.text() {
                    node_text = text.to_string();
                }
            }
        });

        if relation.id == node_id {
            relation.text = node_text;
        }
    }
}
