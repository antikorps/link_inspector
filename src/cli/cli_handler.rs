use std::{
    ffi::OsStr,
    fs::{self, File},
    path::PathBuf,
    process::exit,
};

use axum::body::Bytes;
use clap::Parser;

use crate::{
    app_router::models::{CheckedLink, FileType, NonCheckedLink},
    handlers::{
        html_handler::HtmlHandler, office_handler::OfficeHandler, pdf_handler::PdfHandler,
        txt_handler::TxtHandler,
    },
    http_client,
    link_checker::verifier,
};

/// Verifica el estado de los enlaces de un documento (docx, pptx, xlsx, html, pdf, txt) y genera un informe en csv o json
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Ruta del archivo que se analizará
    #[arg(short, long, default_value = ".")]
    input: PathBuf,

    /// Ruta para el archivo csv resultante (opcional)
    #[arg(short, long, default_value = ".")]
    csv: PathBuf,

    /// Ruta para el archivo json resultante (opcional)
    #[arg(short, long, default_value = ".")]
    json: PathBuf,
}

struct CliHandler {
    input: PathBuf,
    csv: Option<PathBuf>,
    json: Option<PathBuf>,
    bytes: Option<Bytes>,
    filetype: FileType,
    links: Vec<NonCheckedLink>,
    links_verified: Vec<CheckedLink>,
}

fn return_file_writer(path: &PathBuf) -> File {
    match File::create(path.clone()) {
        Err(error) => {
            eprintln!(
                "al crear el archivo de la ruta proporcionada {:?} se ha producido un error {}",
                path, error
            );
            exit(1)
        }
        Ok(ok) => ok,
    }
}

impl CliHandler {
    fn new(arg_input: PathBuf, arg_csv: PathBuf, arg_json: PathBuf) -> CliHandler {
        let csv: Option<PathBuf>;
        let json: Option<PathBuf>;
        if arg_csv == PathBuf::from(".") {
            csv = None;
        } else {
            csv = Some(arg_csv);
        }
        if arg_json == PathBuf::from(".") {
            json = None;
        } else {
            json = Some(arg_json)
        }
        if csv == None && json == None {
            eprintln!("debe proporcionarse un archivo para la salida en formato json o csv");
            exit(1)
        }
        CliHandler {
            input: arg_input,
            csv,
            json,
            bytes: None,
            filetype: FileType::Invalid,
            links: Vec::new(),
            links_verified: Vec::new(),
        }
    }
    fn verify_filetype(&mut self) {
        let valid_filetypes = vec![
            (OsStr::new("html"), FileType::Html),
            (OsStr::new("docx"), FileType::Docx),
            (OsStr::new("pptx"), FileType::Pptx),
            (OsStr::new("xlsx"), FileType::Xlsx),
            (OsStr::new("txt"), FileType::Txt),
            (OsStr::new("pdf"), FileType::Pdf),
        ];

        match self.input.extension() {
            None => {
                eprintln!(
                    "no se ha podido obtener la extensión del archivo facilitado en el input"
                );
                exit(1)
            }
            Some(ok) => {
                for v in valid_filetypes {
                    if v.0 == ok {
                        self.filetype = v.1
                    }
                }
            }
        };

        if self.filetype == FileType::Invalid {
            eprintln!("la extensión facilitada en el archivo facilitado input no es válida");
            exit(1)
        }
    }
    fn get_bytes_input(&mut self) {
        let input_read = match fs::read(self.input.to_owned()) {
            Err(error) => {
                eprintln!(
                    "no ha podido leer el archivo facilitado en el input {}",
                    error
                );
                exit(1)
            }
            Ok(ok) => ok,
        };

        let input_bytes = axum::body::Bytes::from(input_read);
        self.bytes = Some(input_bytes)
    }
    fn get_links(&mut self) {
        let b = self.bytes.as_ref().unwrap().to_owned();

        let result = match self.filetype {
            FileType::Html => HtmlHandler::process_file(b),
            FileType::Docx | FileType::Pptx | FileType::Xlsx => OfficeHandler::process_file(b),
            FileType::Txt => TxtHandler::process_file(b),
            FileType::Pdf => PdfHandler::process_file(b),
            FileType::Invalid => {
                eprintln!("la extensión facilitada en el archivo facilitado input no es válida");
                exit(1)
            }
        };
        self.bytes = None; // variable empty

        let links = match result {
            Err(error) => {
                eprintln!("{}", error);
                exit(1)
            }
            Ok(ok) => ok,
        };
        self.links = links;
    }
    async fn verify_links(&mut self) {
        let client = http_client::create_client::create().await;
        self.links_verified = verifier::verify_links(self.links.clone(), &client).await;
        self.links = Vec::new(); // variable empty
    }
    fn serialize_json(&self) {
        if self.json == None {
            return;
        }
        let json_file = return_file_writer(self.json.as_ref().unwrap());
        match serde_json::to_writer(json_file, &self.links_verified) {
            Err(error) => {
                eprintln!("ha fallado el proceso de serialización y escritura del archivo json con los resultados {error}");
                exit(1)
            }
            Ok(_) => {
                println!("ÉXITO: {} registros serializados en el archivo json", self.links_verified.len());
            },
        }
    }
    fn serialize_csv(&self) {
        if self.csv == None {
            return;
        }
        let csv_file = return_file_writer(self.csv.as_ref().unwrap());
        let mut csv_writer = csv::Writer::from_writer(csv_file);
        let mut errors = false;
        let mut sucess = 0;
        for r in &self.links_verified {
            match csv_writer.serialize(r) {
                Err(error) => {
                    errors = true;
                    eprintln!(
                        "se ha producido un error serializando un registro para el csv {}",
                        error
                    )
                }
                Ok(_) => sucess += 1,
            }
        }
        if errors {
            eprintln!("de {} solo se han serializado {} registro en el archivo csv", self.links_verified.len(), sucess);
            exit(1)
        }
        println!("ÉXITO: {} registros serializados en el archivo csv", sucess);
    }
}

pub async fn manage_cli() {
    let args = Args::parse();
    if args.input == PathBuf::from(".") {
        return;
    }

    let mut cli_handler = CliHandler::new(args.input, args.csv, args.json);
    cli_handler.get_bytes_input();
    cli_handler.verify_filetype();
    cli_handler.get_bytes_input();
    cli_handler.get_links();
    cli_handler.verify_links().await;
    cli_handler.serialize_json();
    cli_handler.serialize_csv();
    
    exit(0)
}