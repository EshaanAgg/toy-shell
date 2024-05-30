use std::env;
use std::path::Path;

pub fn file_exists<'a>(file: &'a str, dirs: Vec<&'a str>) -> Option<String> {
    for dir in dirs {
        let path = format!("{}/{}", dir, file);
        if Path::new(&path).exists() {
            return Some(path);
        }
    }

    None
}

pub fn get_executable<'a>(file: &'a str) -> Option<String> {
    let path = env::var("PATH").unwrap_or_else(|_| "".to_string());
    let dirs = path.split(":").collect::<Vec<&str>>();
    file_exists(file, dirs)
}
