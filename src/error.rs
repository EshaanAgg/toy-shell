use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError<'a> {
    #[error("Error: {0}")]
    GeneralError(String),

    #[error("Invalid Argument '{0}': {1}")]
    ArgsError(&'a str, &'a str),
}
