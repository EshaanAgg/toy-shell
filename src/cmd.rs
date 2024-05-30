enum Commands<'a> {
    Exit(i32),
    Unknown(&'a str),
}

struct Error {
    message: String,
}

impl<'a> Commands<'a> {
    fn from_str(s: &'a str) -> Result<Self, Error> {
        let cmd_parts = s.trim().split(" ").collect::<Vec<&str>>();

        match cmd_parts[0] {
            "exit" => {
                let code = if cmd_parts.len() > 1 {
                    match cmd_parts[1].parse::<i32>() {
                        Ok(c) => c,
                        Err(_) => {
                            return Err(Error {
                                message: "Invalid exit code".to_string(),
                            })
                        }
                    }
                } else {
                    0
                };
                Ok(Commands::Exit(code))
            }
            _ => Ok(Commands::Unknown(s.trim())),
        }
    }
}

pub fn execute(input: &str) {
    match Commands::from_str(input) {
        Ok(cmd) => match cmd {
            Commands::Exit(code) => std::process::exit(code),
            Commands::Unknown(cmd) => println!("Unknown command: {}", cmd),
        },
        Err(e) => println!("Error: {}", e.message),
    }
}
