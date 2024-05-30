use crate::cmd::Commands;
use crate::error::AppError;
use crate::utils;

use std::env;

pub fn parse<'a>(parts: &Vec<&'a str>) -> Result<Commands<'a>, AppError<'a>> {
    if parts.len() < 2 {
        return Err(AppError::ArgsError(
            "command",
            "no command provided to run 'type' on",
        ));
    }

    Ok(Commands::Type(parts[1]))
}

pub fn execute<'a>(cmd: &'a str) {
    let builtins = vec!["echo", "exit", "type"];

    // Builtin commands
    if let Some(_) = builtins.into_iter().find(|&x| x == cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }

    // Executable files in PATH
    let path = env::var("PATH").unwrap_or_else(|_| "".to_string());
    let dirs = path.split(":").collect::<Vec<&str>>();
    if let Some(path) = utils::file_exists(cmd, dirs) {
        println!("{} is {}", cmd, path);
        return;
    }

    println!("{} not found", cmd)
}
