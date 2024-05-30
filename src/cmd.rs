use crate::{commands, error::AppError};

pub enum Commands<'a> {
    Exit(i32),
    Unknown(&'a str),
    Echo(&'a str),
}

impl<'a> Commands<'a> {
    fn from_str(s: &'a str) -> Result<Self, AppError> {
        let cmd_parts = s.trim().split(" ").collect::<Vec<&str>>();

        match cmd_parts[0] {
            "echo" => Ok(Commands::Echo(&s[5..].trim())),
            "exit" => commands::exit::parse(&cmd_parts),
            _ => Ok(Commands::Unknown(s.trim())),
        }
    }
}

pub fn execute(input: &str) {
    match Commands::from_str(input) {
        Ok(cmd) => match cmd {
            Commands::Exit(code) => std::process::exit(code),
            Commands::Unknown(cmd) => println!("{}: command not found", cmd),
            Commands::Echo(str) => println!("{}", str),
        },
        Err(e) => println!("{}", e.to_string()),
    }
}
