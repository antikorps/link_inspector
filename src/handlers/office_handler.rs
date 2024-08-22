use std::io::{Cursor, Read};

use axum::body::Bytes;
use roxmltree::Document;
use zip::ZipArchive;

use crate::app_router::models::NonCheckedLink;

pub struct OfficeHandler {}

struct OfficeSearch {
    zip: ZipArchive<Cursor<Bytes>>,
    files: Vec<FilesRelation>,
    error: Option<String>,
}

struct FilesRelation {
    rels_path: String,
    xml_path: String,
    links: Vec<LinkText>,
}
struct LinkText {
    id: String,
    text: String,
    link: String,
}

impl OfficeSearch {
    fn new(zip: ZipArchive<Cursor<Bytes>>) -> OfficeSearch {
        OfficeSearch {
            zip,
            files: Vec::new(),
            error: None,
        }
    }
    fn get_files(&mut self) {
        for i in 0..self.zip.len() {
            let file = match self.zip.by_index(i) {
                Err(_) => continue,
                Ok(file) => file,
            };
            match Self::check_zip_file_path(&file.name()) {
                None => continue,
                Some(f) => self.files.push(f),
            }
        }
    }

    fn check_zip_file_path(file_path: &str) -> Option<FilesRelation> {
        let mut file_relation = FilesRelation {
            rels_path: String::new(),
            xml_path: String::new(),
            links: Vec::new(),
        };
        if file_path.starts_with("word/_rels/") && file_path.ends_with(".rels") {
            let file_name = file_path
                .strip_prefix("word/_rels/")
                .unwrap()
                .strip_suffix(".rels")
                .unwrap();
            file_relation.rels_path = file_path.to_string();
            file_relation.xml_path = format!("word/{}", file_name);
            return Some(file_relation);
        }

        if file_path.starts_with("ppt/slides/_rels/") && file_path.ends_with(".rels") {
            let file_name = file_path
                .strip_prefix("ppt/slides/_rels/")
                .unwrap()
                .strip_suffix(".rels")
                .unwrap();
            file_relation.rels_path = file_path.to_string();
            file_relation.xml_path = format!("ppt/slides/{}", file_name);
            return Some(file_relation);
        }

        if file_path.starts_with("xl/worksheets/_rels/") && file_path.ends_with(".rels") {
            let file_name = file_path
                .strip_prefix("xl/worksheets/_rels/")
                .unwrap()
                .strip_suffix(".rels")
                .unwrap();
            file_relation.rels_path = file_path.to_string();
            file_relation.xml_path = format!("xl/worksheets/{}", file_name);
            return Some(file_relation);
        }
        return None;
    }

    fn read_rels(&mut self) {
        for f in self.files.iter_mut() {
            let mut file = match self.zip.by_name(&f.rels_path) {
                Err(error) => {
                    let error_message =
                        format!("could not open expected file at {} {}", f.rels_path, error);
                    if self.error.is_some() {
                        self.error.as_mut().unwrap().push_str(&error_message);
                    } else {
                        self.error = Some(error_message)
                    }
                    return;
                }
                Ok(ok) => ok,
            };
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Err(error) => {
                    let error_message =
                        format!("could not read expected file at {} {}", f.rels_path, error);
                    if self.error.is_some() {
                        self.error.as_mut().unwrap().push_str(&error_message);
                    } else {
                        self.error = Some(error_message)
                    }
                    return;
                }
                Ok(_) => (),
            }
            let doc = match Document::parse(&content) {
                Err(error) => {
                    let error_message =
                        format!("could not parse expected file at {} {}", f.rels_path, error);
                    if self.error.is_some() {
                        self.error.as_mut().unwrap().push_str(&error_message);
                    } else {
                        self.error = Some(error_message)
                    }
                    return;
                }
                Ok(ok) => ok,
            };

            for node in doc.descendants() {
                if !node.has_attribute("Target")
                    || !node.has_attribute("TargetMode")
                    || !node.has_attribute("Id")
                {
                    continue;
                }
                f.links.push(LinkText {
                    id: node.attribute("Id").unwrap().to_string(),
                    link: node.attribute("Target").unwrap().to_string(),
                    text: String::new(),
                })
            }
        }
    }

    fn read_xml(&mut self) {
        for f in self.files.iter_mut() {
            let mut file = match self.zip.by_name(&f.xml_path) {
                Err(error) => {
                    let error_message =
                        format!("could not open expected file at {} {}", f.xml_path, error);
                    if self.error.is_some() {
                        self.error.as_mut().unwrap().push_str(&error_message);
                    } else {
                        self.error = Some(error_message)
                    }
                    return;
                }
                Ok(ok) => ok,
            };
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Err(error) => {
                    let error_message =
                        format!("could not read expected file at {} {}", f.xml_path, error);
                    if self.error.is_some() {
                        self.error.as_mut().unwrap().push_str(&error_message);
                    } else {
                        self.error = Some(error_message)
                    }
                    return;
                }
                Ok(_) => (),
            }
            let doc = match Document::parse(&content) {
                Err(error) => {
                    let error_message =
                        format!("could not parse expected file at {} {}", f.xml_path, error);
                    if self.error.is_some() {
                        self.error.as_mut().unwrap().push_str(&error_message);
                    } else {
                        self.error = Some(error_message)
                    }
                    return;
                }
                Ok(ok) => ok,
            };
            // DOCX
            if f.rels_path.starts_with("word") {
                for node in doc.descendants() {
                    if !node.has_tag_name((
                        "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                        "hyperlink",
                    )) {
                        continue;
                    }

                    let id = match node.attribute((
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                        "id",
                    )) {
                        None => continue,
                        Some(s) => s,
                    };

                    let mut text: Option<String> = None;
                    for subnode in node.descendants() {
                        if !subnode.has_tag_name((
                            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                            "t",
                        )) {
                            continue;
                        }
                        let subnode_text = subnode.text();
                        if subnode_text.is_none() {
                            continue;
                        }
                        if text.is_none() {
                            text = Some(subnode_text.unwrap().to_string())
                        } else {
                            text.clone().unwrap().push_str(subnode_text.unwrap())
                        }
                    }
                    if text.is_some() {
                        for l in f.links.iter_mut() {
                            if l.id == id {
                                l.text = text.clone().unwrap().to_string()
                            }
                        }
                    }
                }
            }
            // PPTX
            if f.rels_path.starts_with("ppt") {
                for node in doc.descendants() {
                    if !node.has_tag_name((
                        "http://schemas.openxmlformats.org/drawingml/2006/main",
                        "r",
                    )) {
                        continue;
                    }

                    let mut id = None;

                    for subnode in node.descendants() {
                        if !subnode.has_tag_name((
                            "http://schemas.openxmlformats.org/drawingml/2006/main",
                            "hlinkClick",
                        )) {
                            continue;
                        };
                        id = subnode.attribute((
                            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                            "id",
                        ))
                    }
                    if id.is_none() {
                        continue;
                    }

                    let mut text: Option<String> = None;
                    for subnode in node.descendants() {
                        if !subnode.has_tag_name((
                            "http://schemas.openxmlformats.org/drawingml/2006/main",
                            "t",
                        )) {
                            continue;
                        }
                        let subnode_text = subnode.text();
                        if subnode_text.is_none() {
                            continue;
                        }
                        if text.is_none() {
                            text = Some(subnode_text.unwrap().to_string())
                        } else {
                            text.clone().unwrap().push_str(subnode_text.unwrap())
                        }
                    }
                    if text.is_some() {
                        for l in f.links.iter_mut() {
                            if l.id == id.unwrap() {
                                l.text = text.clone().unwrap().to_string()
                            }
                        }
                    }
                }
            }

            // XLSX
            if f.rels_path.starts_with("xl") {
                for node in doc.descendants() {
                    if !node.has_tag_name((
                        "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                        "hyperlink",
                    )) {
                        continue;
                    }

                    let id = match node.attribute((
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                        "id",
                    )) {
                        None => continue,
                        Some(s) => s,
                    };

                    let display = match node.attribute("display") {
                        None => continue,
                        Some(s) => s,
                    };

                    let cell_ref = match node.attribute("ref") {
                        None => "",
                        Some(s) => s,
                    };
                    for l in f.links.iter_mut() {
                        if l.id == id {
                            l.text = format!("{} ({})", display, cell_ref);
                        }
                    }
                }
            }
        }
    }
    fn extract_result(self) -> Result<Vec<NonCheckedLink>, String> {
        if self.error.is_some() {
            return Err(self.error.unwrap());
        }
        let mut non_checked_links = Vec::new();
        for f in self.files {
            for l in f.links {
                non_checked_links.push(NonCheckedLink {
                    url: l.link,
                    text: l.text,
                })
            }
        }
        return Ok(non_checked_links);
    }
}

/*
Dentro del ZIP se encuentran los documentos con extension .rels en los que se guarda la información de los enlaces
La información de los enlaces no contiene el texto, que debe buscarse en el archivo del mismo nombre con extensión xmls

DOCX:
word/rels/document.xml.rels
word/document.xml

PPTX:
ppt/slides/_rels/slide1.xml.rels
ppt/slides/slide1.xml

XLSX:
xl/worksheets/_rels/workbook.xml.rels
xl/worksheets/workbook.xml

1. Buscar esos archivos para crear los FilesRelation (get_files)
2. Parsear .rels (read_rels) para encontrar los enlaces
3. Parsear .xml para encontrar los textos y relacionar (read_xml)
4. Ofrecer resultados  Result<Vec<NonCheckedLink>, String (extract_result)

*/

impl OfficeHandler {
    pub fn process_file(file_bytes: Bytes) -> Result<Vec<NonCheckedLink>, String> {
        let cursor = Cursor::new(file_bytes);
        let zip: ZipArchive<Cursor<Bytes>> = match ZipArchive::new(cursor) {
            Err(error) => {
                let error_message = format!("could not read office file as a zip: {error}");
                return Err(error_message);
            }
            Ok(ok) => ok,
        };
        let mut office_search_handler = OfficeSearch::new(zip);
        office_search_handler.get_files();
        office_search_handler.read_rels();
        office_search_handler.read_xml();
        office_search_handler.extract_result()
    }
}
