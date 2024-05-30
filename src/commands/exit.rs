use crate::cmd::Commands;
use crate::error::Error;

pub fn parse<'a>(parts: &Vec<&'a str>) -> Result<Commands<'a>, Error> {
    let code = if parts.len() > 1 {
        match parts[1].parse::<i32>() {
            Ok(c) => c,
            Err(_) => return Err(Error::new("Invalid exit code provided".to_string())),
        }
    } else {
        0
    };

    Ok(Commands::Exit(code))
}
