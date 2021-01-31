use crate::{
    colors::{RED, YELLOW},
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

pub fn get_document(path: &PathBuf, output: &mut Output) -> Document {
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

fn file_contents(path: &PathBuf, contents: String, output: &mut Output) -> Document {
    contents.parse::<Document>().unwrap_or_else(|e| {
        output.notify_error(
            ErrorCode::ParseError,
            &[
                RED.bold().paint("Failed to parse "),
                YELLOW.bold().paint(path.to_string_lossy()),
                RED.bold().paint(": \n"),
                RED.paint(e.to_string()),
            ],
        );
        Document::new()
    })
}

pub fn write_document(file: &PathBuf, document: &Document, output: &mut Output) {
    if let Ok(file) = OpenOptions::new().write(true).open(&file) {
        let mut buffer = BufWriter::new(file);
        buffer
            .write_all(document.to_string().as_ref())
            .unwrap_or_else(|_| {
                output.notify_error(
                    ErrorCode::WriteError,
                    &[RED.bold().paint("Failed to write settings")],
                );
            });
        buffer.flush().unwrap_or_else(|_| {
            output.notify_error(
                ErrorCode::WriteError,
                &[RED.bold().paint("Failed to write settings")],
            );
        });
    } else {
        output.notify_error(
            ErrorCode::WriteError,
            &[
                RED.bold().paint("Could not open"),
                YELLOW.bold().paint(file.to_string_lossy()),
            ],
        );
    }
}
