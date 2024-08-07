// document.xml.rels
use super::process::HandlerDocx;
use std::io::{Cursor, Read};
use zip::ZipArchive;

impl HandlerDocx {
    pub fn get_xml_rels(&mut self) {
        let cursor = Cursor::new(&self.file_bytes);

        let mut zip;
        match ZipArchive::new(cursor) {
            Err(error) => {
                let error_message = format!("could not read docx as a zip file {error}");
                self.error = Some(error_message);
                return;
            }
            Ok(ok) => zip = ok,
        }

        for i in 0..zip.len() {
            let mut file;
            match zip.by_index(i) {
                Err(_) => continue,
                Ok(ok) => file = ok,
            }
            if !file.name().starts_with("word/_rels/") {
                continue;
            }
            let mut xml_rels = String::new();
            match file.read_to_string(&mut xml_rels) {
                Err(error) => {
                    let error_message = format!("could not read document.xmls.rels {error}");
                    self.error = Some(error_message);
                    return;
                }
                Ok(_) => self.xml_rels.push(xml_rels),
            }
        }
    }
}
