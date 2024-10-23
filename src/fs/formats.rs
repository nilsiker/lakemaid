use std::ffi::OsStr;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileFormatError {
    #[error("Failed to parse file format.")]
    Parse,
}

#[derive(Debug)]
pub enum FileFormat {
    Rust,
}

impl TryFrom<&OsStr> for FileFormat {
    type Error = FileFormatError;

    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        match value.to_str() {
            Some("rs") => Ok(Self::Rust),
            _ => Err(FileFormatError::Parse),
        }
    }
}
