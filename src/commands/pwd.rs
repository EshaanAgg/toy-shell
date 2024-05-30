use crate::cmd::Commands;
use crate::error::AppError;

pub fn parse<'a>(parts: &Vec<&'a str>) -> Result<Commands<'a>, AppError<'a>> {
    if parts.len() != 1 {
        return Err(AppError::GeneralError(format!(
            "too many arguments, expected no args, got {}",
            parts.len() - 1
        )));
    }

    Ok(Commands::Pwd)
}

pub fn execute() {
    println!(
        "{}",
        std::env::current_dir()
            .expect("failed to fetch the current working directory")
            .display()
    )
}
