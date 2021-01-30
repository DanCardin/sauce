use crate::{
    colors::{NORMAL, RED, YELLOW},
    output::{ErrorCode, Output},
};
use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read},
};
use std::{
    io::Write,
    path::{Path, PathBuf},
};
use toml_edit::Document;

pub fn get_document<'a>(path: &'a PathBuf, output: &'a mut Output<'a>) -> Document {
    let content = read_file(path);
    file_contents(path, content, output)
}

fn read_file(path: &Path) -> String {
    if let Ok(file) = std::fs::File::open(path) {
        let mut reader = BufReader::new(file);

        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap_or(0);
        contents
    } else {
        String::new()
    }
}

fn file_contents<'a>(path: &'a PathBuf, contents: String, output: &'a mut Output<'a>) -> Document {
    contents.parse::<Document>().unwrap_or_else(|e| {
        output.push_error(
            ErrorCode::ParseError,
            &[
                RED.bold().paint("Failed to parse "),
                YELLOW.bold().paint(path.to_string_lossy()),
                NORMAL.paint(format!(": {}", e)),
            ],
        );
        Document::new()
    })
}

pub fn write_document<'a>(file: &'a PathBuf, document: &Document, output: &'a mut Output<'a>) {
    if let Ok(file) = OpenOptions::new().write(true).open(&file) {
        let mut buffer = BufWriter::new(file);
        buffer
            .write_all(document.to_string().as_ref())
            .expect(&RED.bold().paint("Failed to write settings"));
        buffer
            .flush()
            .expect(&RED.bold().paint("Failed to write settings"));
    } else {
        output.push_error(
            ErrorCode::WriteError,
            &[
                RED.bold().paint("Could not open "),
                YELLOW.bold().paint(file.to_string_lossy()),
            ],
        );
    }
}
