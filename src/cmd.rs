enum Commands<'a> {
    Unknown(&'a str),
}

impl<'a> Commands<'a> {
    fn from_str(s: &'a str) -> Self {
        match s {
            _ => Commands::Unknown(s),
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
