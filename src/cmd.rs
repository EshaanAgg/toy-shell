use crate::utils;
use crate::{commands, error::AppError};

use std::process::Command;

pub enum Commands<'a> {
    Exit(i32),
    Echo(&'a str),
    Type(&'a str),
    Unknown(Vec<&'a str>),
}

impl<'a> Commands<'a> {
    fn from_str(s: &'a str) -> Result<Self, AppError> {
        let cmd_parts = s.trim().split(" ").collect::<Vec<&str>>();

        match cmd_parts[0] {
            "echo" => Ok(Commands::Echo(&s[5..].trim())),
            "exit" => commands::exit::parse(&cmd_parts),
            "type" => commands::typ::parse(&cmd_parts),
            _ => Ok(Commands::Unknown(cmd_parts)),
        }
    }
}

pub fn execute(input: &str) {
    match Commands::from_str(input) {
        Ok(cmd) => match cmd {
            Commands::Echo(str) => println!("{}", str),
            Commands::Exit(code) => std::process::exit(code),
            Commands::Type(str) => commands::typ::execute(str),
            Commands::Unknown(cmd) => {
                // If it is an executable, run the same
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
