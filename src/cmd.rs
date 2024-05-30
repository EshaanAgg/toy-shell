enum Commands<'a> {
    Unknown(&'a str),
}

impl<'a> Commands<'a> {
    fn from_str(s: &'a str) -> Self {
        let trimmed_cmd = s.trim();

        match trimmed_cmd {
            _ => Commands::Unknown(trimmed_cmd),
        }
    }
}

pub fn execute(input: &str) {
    match Commands::from_str(input) {
        Commands::Unknown(s) => {
            println!("{}: command not found", s);
        }
    }
}
