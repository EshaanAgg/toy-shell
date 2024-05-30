use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError<'a> {
    // #[error("Error: {0}")]
    // GeneralError(&'a str),
    #[error("Invalid Argument '{0}': {1}")]
    ArgsError(&'a str, &'a str),
}
