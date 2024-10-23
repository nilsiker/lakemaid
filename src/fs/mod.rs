use std::{io::Read, path::Path};

use formats::FileFormat;

pub mod formats;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Error when reading file: {0}")]
    File(#[from] std::io::Error),
    #[error("Unsupported file type '{0}'")]
    UnsupportedFile(String),
    #[error("Could not determine file type for path '{0}'")]
    InvalidFileType(String),
}

pub fn get_contents_and_format(input: &str) -> Result<(String, FileFormat), FileError> {
    let path = Path::new(input);

    let Some(extension) = path.extension() else {
        return Err(FileError::InvalidFileType(input.to_string()));
    };

    let Ok(file_format) = FileFormat::try_from(extension) else {
        return Err(FileError::UnsupportedFile(
            extension.to_string_lossy().to_string(),
        ));
    };

    let mut file = std::fs::File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok((contents, file_format))
}
