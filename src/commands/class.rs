use thiserror::Error;

use crate::{fs::{self, formats::FileFormat}, mermaid::class_diagram};

#[derive(Error, Debug)]
pub enum ClassError {
    #[error(transparent)]
    File(#[from] fs::FileError),
    #[error("Unable to generate class diagram (UNSUPPORTED FILE FORMAT).")]
    NotSupported(FileFormat),
}

pub fn exec(input: &str) -> Result<(), ClassError> {
    let (contents, format) = fs::get_contents_and_format(input)?;

    let class_diagram = match format {
        fs::formats::FileFormat::Rust => class_diagram::rs::parse(&contents),
    };

    println!("{}", String::from(class_diagram));
    Ok(())
}
