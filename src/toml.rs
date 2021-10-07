use crate::{
    colors::{RED, YELLOW},
    output::{ErrorCode, Output},
};
use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read},
    str::FromStr,
};
use std::{io::Write, path::Path};
use toml_edit::{Document, Item, Table, Value};

pub fn get_document(path: &Path, output: &mut Output) -> Document {
    let content = read_file(path);
    file_contents(path, content, output)
}

pub fn write_document(file: &Path, document: &Document, output: &mut Output) {
    let handle = OpenOptions::new().write(true).open(&file);
    write_contents(handle, file, document, output);
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

fn file_contents(path: &Path, contents: String, output: &mut Output) -> Document {
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

pub fn write_contents<W: Write>(
    handle: Result<W, std::io::Error>,
    file: &Path,
    document: &Document,
    output: &mut Output,
) {
    if let Ok(f) = handle {
        let mut buffer = BufWriter::new(f);
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
                RED.bold().paint("Could not open "),
                YELLOW.bold().paint(file.to_string_lossy()),
            ],
        );
    }
}

pub fn ensure_section<'a>(document: &'a mut Document, section: &str) -> &'a mut Item {
    let env_section = document.as_table_mut().entry(section);
    if env_section.is_none() {
        *env_section = Item::Table(Table::new());
    }
    env_section
}

pub fn value_from_string(raw_value: &str) -> Item {
    let value = Value::from_str(raw_value).unwrap_or_else(|_| Value::from(raw_value));
    toml_edit::value(value)
}

pub fn unwrap_toml_value(value: &Value) -> String {
    match value {
        Value::InlineTable(_) => value.as_inline_table().unwrap().to_string(),
        Value::Array(_) => value.as_array().unwrap().to_string(),
        Value::String(_) => value.as_str().unwrap().to_string(),
        Value::Integer(_) => value.as_integer().unwrap().to_string(),
        Value::Boolean(_) => value.as_bool().unwrap().to_string(),
        Value::Float(_) => value.as_float().unwrap().to_string(),
        Value::DateTime(_) => value.as_date_time().unwrap().to_string(),
    }
}

#[cfg(test)]
mod tests {
    mod write_contents {
        use crate::test_utils::setup;

        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_writes_contents() {
            let mut file = Vec::new();

            let toml = r#"
            [foo]
            bar = "baz"
            "#;
            let document = toml.parse::<Document>().expect("invalid doc");

            let (_, _, mut output) = setup();

            write_contents(
                Ok(&mut file),
                Path::new("test.toml"),
                &document,
                &mut output,
            );
            assert_eq!(std::str::from_utf8(&file).unwrap(), toml.to_string());
        }

        #[test]
        fn it_fails_on_file_error() {
            let document = "".parse::<Document>().expect("invalid doc");

            let (_, err, mut output) = setup();

            write_contents::<Vec<u8>>(
                Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")),
                Path::new("test.toml"),
                &document,
                &mut output,
            );
            assert_eq!(err.value(), "Could not open test.toml\n");
        }
    }
}
