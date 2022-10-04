use crate::toml::{ensure_section, write_document};
use crate::{
    colors::{BLUE, RED, TABLE_BLUE, TABLE_YELLOW, YELLOW},
    toml::unwrap_toml_value,
};
use anyhow::Result;
use comfy_table::{Attribute, Cell, ContentArrangement, Row, Table};
use std::{fmt::Display, io::Write, ops::Deref, path::Path};
use toml_edit::{Document, Item};

use ansi_term::{ANSIString, ANSIStrings};

pub struct Output {
    out: Box<dyn Write>,
    err: Box<dyn Write>,
    color: bool,
    quiet: bool,
    verbose: bool,
    show: bool,
    code: Option<ErrorCode>,
}

impl std::fmt::Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Output")
            .field("color", &self.color)
            .field("quiet", &self.code)
            .field("verbose", &self.code)
            .field("show", &self.code)
            .field("code", &self.code)
            .finish()
    }
}

impl Output {
    pub fn new(out: Box<dyn Write>, err: Box<dyn Write>) -> Self {
        Self {
            out,
            err,
            color: false,
            quiet: false,
            verbose: false,
            show: false,
            code: None,
        }
    }

    pub fn quiet(mut self, value: bool) -> Self {
        self.set_quiet(value);
        self
    }

    pub fn set_quiet(&mut self, value: bool) {
        self.quiet = value;
    }

    pub fn verbose(mut self, value: bool) -> Self {
        self.set_verbose(value);
        self
    }

    pub fn set_verbose(&mut self, value: bool) {
        self.verbose = value;
    }

    pub fn color(mut self, value: bool) -> Self {
        self.color = value;
        self
    }

    pub fn show(mut self, value: bool) -> Self {
        self.show = value;
        self
    }

    pub fn set_show(&mut self, value: bool) {
        self.show = value;
    }

    fn format(&self, output: impl Display) -> String {
        let mut result = format!("{}", output);
        if !result.is_empty() {
            result.push('\n');
        }
        result
    }

    pub fn format_table(
        &self,
        headers: &[&str],
        data: Vec<Vec<&str>>,
        preset: Option<&str>,
    ) -> String {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);
        if let Some(preset) = preset {
            table.load_preset(preset);
        } else {
            table.load_preset("││──╞═╪╡│    ┬┴┌┐└┘");
        }
        table.set_header(headers);
        table.enforce_styling();

        for data_row in data {
            let mut row = Row::new();
            for (i, data_cell) in data_row.iter().enumerate() {
                let mut cell = Cell::new(data_cell);

                if self.color {
                    if i == 0 {
                        cell = cell.add_attribute(Attribute::Bold).fg(TABLE_BLUE);
                    } else {
                        cell = cell.fg(TABLE_YELLOW);
                    }
                }
                row.add_cell(cell);
            }
            table.add_row(row);
        }

        self.format(table)
    }

    pub fn output(&mut self, output: impl Display) -> bool {
        let data = self.format(output);

        let stream = if self.show {
            &mut self.err
        } else {
            &mut self.out
        };
        let result = stream.write_all(data.as_bytes()).is_ok();

        if self.verbose {
            self.err
                .write_all(data.as_bytes())
                .expect("Couldn't write verbose output");
        }
        result
    }

    pub fn notify(&mut self, message: &[ANSIString]) -> String {
        let message = if self.color {
            self.format(ANSIStrings(message))
        } else {
            self.format(
                message
                    .iter()
                    .map(|f| f.deref())
                    .collect::<Vec<&str>>()
                    .join(""),
            )
        };

        self.notify_str(&message)
    }

    pub fn notify_str(&mut self, message: &str) -> String {
        let message_content = message.as_bytes();
        if self.quiet {
            String::new()
        } else {
            self.err.write_all(message_content).ok();
            message.to_string()
        }
    }

    pub fn notify_error(&mut self, code: ErrorCode, message: &[ANSIString]) -> String {
        self.code = Some(code);
        self.notify(message)
    }

    pub fn error_code(&self) -> Option<i32> {
        self.code.clone().map(|c| c as i32)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.out.flush()?;
        self.err.flush()?;
        Ok(())
    }

    pub fn write_toml<I, T>(
        &mut self,
        file: &Path,
        document: &mut Document,
        heading: &str,
        values: I,
    ) where
        I: IntoIterator<Item = (T, Item)>,
        T: AsRef<str>,
    {
        for (name, value) in values.into_iter() {
            self.notify(&[
                "Setting ".into(),
                BLUE.bold().paint(name.as_ref()),
                " = ".into(),
                YELLOW.paint(unwrap_toml_value(value.as_value().unwrap())),
            ]);

            if !self.show {
                let section = ensure_section(document, heading);
                section[name.as_ref()] = value;
            }
        }

        if !self.show {
            write_document(file, document, self);
        }
    }

    pub fn create_file(&mut self, file: &Path) -> Result<(), String> {
        if let Some(parent) = file.parent() {
            let created = if self.show {
                true
            } else {
                std::fs::create_dir_all(parent).is_ok()
            };

            if !created {
                return Err(self.notify_error(
                    ErrorCode::WriteError,
                    &[
                        RED.paint("Couldn't create "),
                        YELLOW.paint(parent.to_string_lossy()),
                    ],
                ));
            }
        }

        if file.is_file() {
            return Err(self.notify_error(
                ErrorCode::WriteError,
                &[
                    RED.bold().paint("File already exists at "),
                    YELLOW.paint(file.to_string_lossy()),
                ],
            ));
        }

        let created = if self.show {
            true
        } else {
            std::fs::File::create(file).is_ok()
        };

        if !created {
            return Err(self.notify_error(
                ErrorCode::WriteError,
                &[
                    RED.bold().paint("Couldn't create "),
                    YELLOW.paint(file.to_string_lossy()),
                ],
            ));
        }

        self.notify(&[
            BLUE.bold().paint("Created "),
            YELLOW.paint(file.to_string_lossy()),
        ]);
        Ok(())
    }

    pub fn move_file(&mut self, source: &Path, dest: &Path, copy: bool) -> Result<(), String> {
        let moved = if self.show {
            true
        } else {
            let result = if copy {
                std::fs::copy(source, dest).map(|_| ())
            } else {
                std::fs::rename(source, dest)
            };
            result.is_ok()
        };

        if moved {
            let message = &[
                BLUE.paint("Moved "),
                YELLOW.paint(source.to_string_lossy()),
                BLUE.paint(" to "),
                YELLOW.paint(dest.to_string_lossy()),
            ];
            self.notify(message);
            Ok(())
        } else {
            let message = &[
                RED.paint("Failed to move "),
                YELLOW.paint(source.to_string_lossy()),
                RED.paint(" to "),
                YELLOW.paint(dest.to_string_lossy()),
            ];
            Err(self.notify_error(ErrorCode::WriteError, message))
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorCode {
    WriteError = 1,
    ParseError = 2,
}
