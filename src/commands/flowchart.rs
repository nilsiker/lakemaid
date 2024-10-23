use thiserror::Error;

use crate::fs::formats::FileFormat;

#[derive(Error, Debug)]
pub enum FlowchartError {
    #[error(transparent)]
    File(#[from] crate::fs::FileError),
    #[error("Unable to generate class diagram (UNSUPPORTED FILE FORMAT).")]
    NotSupported(FileFormat),
}

pub fn exec(input: &str) -> Result<(), FlowchartError> {
    let (contents, format) = crate::fs::get_contents_and_format(input)?;

    let flowchart = match format {
        _ => return Err(FlowchartError::NotSupported(format)),
    };

    Ok(())
}
