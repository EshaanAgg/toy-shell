use crate::utils;
use crate::{commands, error::AppError};

use std::process::Command;

pub enum Commands<'a> {
    Exit(i32),
    Echo(&'a str),
    Type(&'a str),
    Cd(&'a str),
    Pwd,
    Unknown(Vec<&'a str>),
}

impl<'a> Commands<'a> {
    fn from_str(s: &'a str) -> Result<Self, AppError> {
        let cmd_parts = s.trim().split(" ").collect::<Vec<&str>>();

        match cmd_parts[0] {
            "cd" => commands::cd::parse(&cmd_parts),
            "pwd" => commands::pwd::parse(&cmd_parts),
            "type" => commands::typ::parse(&cmd_parts),
            "exit" => commands::exit::parse(&cmd_parts),
            "echo" => Ok(Commands::Echo(&s[5..].trim())),
            _ => Ok(Commands::Unknown(cmd_parts)),
        }
    }
}

pub fn execute(input: &str) {
    match Commands::from_str(input) {
        Ok(cmd) => match cmd {
            Commands::Echo(str) => println!("{}", str),
            Commands::Exit(code) => std::process::exit(code),

            Commands::Pwd => commands::pwd::execute(),
            Commands::Type(str) => commands::typ::execute(str),
            Commands::Cd(str) => commands::cd::execute(str),

            Commands::Unknown(cmd) => {
                // Check if it is an executable on the system
                if let Some(path) = utils::get_file(cmd[0]) {
                    let mut child = Command::new(path)
                        .args(&cmd[1..])
                        .stdout(std::io::stdout())
                        .spawn()
                        .expect("Failed to execute the binary");

                    child
                        .wait()
                        .expect("The child process ran into an error during execution");
                } else {
                    // Print unknown command message
                    println!("{}: command not found", cmd[0]);
                }
            }
        },
        Err(e) => println!("{}", e.to_string()),
    }
}
