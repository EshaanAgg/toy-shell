use crate::cmd::Commands;
use crate::error::AppError;
use crate::utils;

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
    if let Some(path) = utils::get_executable(cmd) {
        println!("{} is {}", cmd, path);
        return;
    }

    println!("{} not found", cmd)
}
