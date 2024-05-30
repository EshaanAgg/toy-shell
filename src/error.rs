pub struct Error {
    message: String,
}

impl Error {
    pub fn new(s: String) -> Self {
        Error { message: s }
    }

    pub fn show(&self) {
        println!("Error: {}", self.message);
    }
}
