use std::path::{Path, PathBuf};
use std::io::{BufReader, Read};
use toml_edit::Document;

pub fn get_document(path: &PathBuf) -> Document {
    let content = read_file(path);
    file_contents(path, content)
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

fn file_contents(path: &PathBuf, contents: String) -> Document {
    contents.parse::<Document>().unwrap_or_else(|e| {
        eprintln!("Failed to parse {}: {}", path.to_string_lossy(), e);
        Document::new()
    })
}

