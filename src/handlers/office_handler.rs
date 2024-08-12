use std::io::{Cursor, Read};

use axum::body::Bytes;
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
    pub fn get_links(
        mut zip: ZipArchive<Cursor<Bytes>>,
        checked_file_type: FileType,
    ) -> Result<Vec<NonCheckedLink>, String> {
        let mut relations_link_text = Vec::new();
        let mut content_text_files = Vec::new();

        for i in 0..zip.len() {
            let mut file;

            match zip.by_index(i) {
                Err(_) => continue,
                Ok(ok) => file = ok,
            }

            match checked_file_type {
                FileType::Docx => {
                    if !file.name().starts_with("word/_rels/") {
                        continue;
                    }
                }
                FileType::Pptx => {
                    if !file.name().starts_with("ppt/slides/_rels") {
                        continue;
                    }
                }
                FileType::Xlsx => {
                    if !file.name().starts_with("xl/worksheets/_rels") {
                        continue;
                    }
                }
                _ => continue,
            }
            let mut xml_rels = String::new();
            if let Err(error) = file.read_to_string(&mut xml_rels) {
                let error_message = format!("could not read xmls.rels {error}");
                return Err(error_message);
            }

            let doc = match roxmltree::Document::parse(&xml_rels) {
                Err(error) => {
                    let error_message = format!("could not parse xml on xml_rels string: {error}");
                    return Err(error_message);
                }
                Ok(ok) => ok,
            };

            let _ = doc.descendants().find(|f| {
                if f.attribute("TargetMode").is_none() {
                    return false;
                };
                if f.attribute("TargetMode").unwrap() != "External" {
                    return false;
                }
                if f.attribute("Target").is_none() {
                    return false;
                }
                if f.attribute("Id").is_none() {
                    return false;
                }

                let id = f.attribute("Id").unwrap().to_string();
                let link = f.attribute("Target").unwrap().to_string();
                let text = String::new();
                let content_text_file = file
                    .name()
                    .to_string()
                    .strip_suffix(".rels")
                    .unwrap()
                    .to_string();
                content_text_files.push(content_text_file.clone());
                relations_link_text.push(OfficeRelationLinkText {
                    content_text_file,
                    id,
                    text,
                    link,
                });
                false
            });
        }

        // Iterate text files (no _rels)
        for content_file in content_text_files {
            match checked_file_type {
                FileType::Docx => {
                    let content_file_path =
                        format!("word/{}", content_file.strip_prefix("word/_rels/").unwrap());
                    let mut content_file_zip;
                    match zip.by_name(&content_file_path) {
                        Err(_) => continue,
                        Ok(ok) => content_file_zip = ok,
                    }
                    let mut content_file_xml = String::new();
                    if let Err(error) = content_file_zip.read_to_string(&mut content_file_xml) {
                        let error_message = format!("could not read {content_file_path} {error}");
                        return Err(error_message);
                    }

                    let doc = match roxmltree::Document::parse(&content_file_xml) {
                        Err(error) => {
                            let error_message = format!(
                                "could not parse xml on {content_file_path} string: {error}"
                            );
                            return Err(error_message);
                        }
                        Ok(ok) => ok,
                    };

                    for node in doc.descendants() {
                        if node.has_tag_name((
                            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                            "hyperlink",
                        )) {
                            let mut node_id = String::new();
                            let mut node_text = String::new();
                            node.descendants().find(|node_des| {

                                match node_des.attribute(("http://schemas.openxmlformats.org/officeDocument/2006/relationships", "id")) {
                                    None => (),
                                    Some(ok) => node_id = ok.to_string(),
                                }

                                if node_des.has_tag_name(("http://schemas.openxmlformats.org/wordprocessingml/2006/main", "t")) {
                                    node_text = node_des.text().unwrap().to_string();
                                }

                                false
                            });

                            for r in relations_link_text.iter_mut() {
                                if r.content_text_file != content_file {
                                    continue;
                                }
                                if r.id != node_id {
                                    continue;
                                }
                                r.text = node_text.clone();
                            }
                        }
                    }
                }
                FileType::Pptx => {
                    let content_file_path = format!(
                        "ppt/slides/{}",
                        content_file.strip_prefix("ppt/slides/_rels/").unwrap()
                    );
                    let mut content_file_zip;
                    match zip.by_name(&content_file_path) {
                        Err(_) => continue,
                        Ok(ok) => content_file_zip = ok,
                    }
                    let mut content_file_xml = String::new();
                    if let Err(error) = content_file_zip.read_to_string(&mut content_file_xml) {
                        let error_message = format!("could not read {content_file_path} {error}");
                        return Err(error_message);
                    }

                    let doc = match roxmltree::Document::parse(&content_file_xml) {
                        Err(error) => {
                            let error_message = format!(
                                "could not parse xml on {content_file_path} string: {error}"
                            );
                            return Err(error_message);
                        }
                        Ok(ok) => ok,
                    };

                    for node in doc.descendants() {
                        if node.has_tag_name((
                            "http://schemas.openxmlformats.org/drawingml/2006/main",
                            "r",
                        )) {
                            let mut node_id = String::new();
                            let mut node_text = String::new();
                            node.descendants().find(|node_des| {

                                match node_des.attribute(("http://schemas.openxmlformats.org/officeDocument/2006/relationships", "id")) {
                                    None => (),
                                    Some(ok) => node_id = ok.to_string(),
                                }

                                if node_des.has_tag_name(("http://schemas.openxmlformats.org/drawingml/2006/main", "t")) {
                                    node_text = node_des.text().unwrap().to_string();
                                }

                                false
                            });

                            for r in relations_link_text.iter_mut() {
                                if r.content_text_file != content_file {
                                    continue;
                                }
                                if r.id != node_id {
                                    continue;
                                }
                                r.text = node_text.clone();
                            }
                        }
                    }
                }

                FileType::Xlsx => {
                    let content_file_path = format!(
                        "xl/worksheets/{}",
                        content_file.strip_prefix("xl/worksheets/_rels/").unwrap()
                    );
                    let mut content_file_zip;
                    match zip.by_name(&content_file_path) {
                        Err(_) => continue,
                        Ok(ok) => content_file_zip = ok,
                    }
                    let mut content_file_xml = String::new();
                    if let Err(error) = content_file_zip.read_to_string(&mut content_file_xml) {
                        let error_message = format!("could not read {content_file_path} {error}");
                        return Err(error_message);
                    }

                    let doc = match roxmltree::Document::parse(&content_file_xml) {
                        Err(error) => {
                            let error_message = format!(
                                "could not parse xml on {content_file_path} string: {error}"
                            );
                            return Err(error_message);
                        }
                        Ok(ok) => ok,
                    };

                    for node in doc.descendants() {
                        if node.has_tag_name((
                            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                            "hyperlink",
                        )) {
                            let mut node_id = String::new();
                            let mut node_text = String::new();
                            let mut cell_ref = String::new();

                            match node.attribute(("http://schemas.openxmlformats.org/officeDocument/2006/relationships", "id")) {
                                None => (),
                                Some(ok) => node_id = ok.to_string(),
                            }

                            match node.attribute("ref") {
                                None => (),
                                Some(ok) => cell_ref = ok.to_string(),
                            }

                            match node.attribute("display") {
                                None => (),
                                Some(ok) => node_text = ok.to_string(),
                            }

                            node_text.push_str(&format!(" ({})", cell_ref));

                            for r in relations_link_text.iter_mut() {
                                if r.content_text_file != content_file {
                                    continue;
                                }
                                if r.id != node_id {
                                    continue;
                                }
                                r.text = node_text.clone();
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        let mut non_checked_links = Vec::new();
        for r in relations_link_text {
            non_checked_links.push(NonCheckedLink {
                url: r.link,
                text: r.text,
            })
        }
        Ok(non_checked_links)
    }

    pub fn process_file(
        file_bytes: Bytes,
        checked_file_type: FileType,
    ) -> Result<Vec<NonCheckedLink>, String> {
        let cursor = Cursor::new(file_bytes);

        match ZipArchive::new(cursor) {
            Err(error) => {
                let error_message = format!("could not read docx as a zip file {error}");
                Err(error_message)
            }
            Ok(zip) => Self::get_links(zip, checked_file_type),
        }
    }
}
