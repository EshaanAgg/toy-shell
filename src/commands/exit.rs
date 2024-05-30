use crate::cmd::Commands;
use crate::error::AppError;

pub fn parse<'a>(parts: &Vec<&'a str>) -> Result<Commands<'a>, AppError<'a>> {
    let code = if parts.len() > 1 {
        match parts[1].parse::<i32>() {
            Ok(c) => c,
            Err(_) => return Err(AppError::ArgsError("exit_code", "can't convert to i32")),
        }
    } else {
        0
    };

    Ok(Commands::Exit(code))
}
