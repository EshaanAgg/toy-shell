use crate::cmd::Commands;
use crate::error::AppError;

use std::env;
use std::path::Path;

pub fn parse<'a>(parts: &Vec<&'a str>) -> Result<Commands<'a>, AppError<'a>> {
    if parts.len() < 2 {
        return Err(AppError::ArgsError(
            "location",
            "need to provide a location to change directory to",
        ));
    }

    if parts.len() > 2 {
        return Err(AppError::GeneralError(format!(
            "too many arguments, expected 1, got {}",
            parts.len() - 1
        )));
    }

    Ok(Commands::Cd(parts[1]))
}

pub fn execute<'a>(path: &'a str) {
    let home_path = env::var("HOME").unwrap_or_else(|_| "/".to_string());
    let path_buf = Path::new(match path {
        "~" => home_path.as_str(),
        _ => path,
    });

    if !path_buf.exists() {
        println!("cd: {}: No such file or directory", path);
        return;
    }

    env::set_current_dir(&path_buf).expect("failed to change path to a valid directory")
}
