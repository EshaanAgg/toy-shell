use std::env;
use std::path::Path;

/// Checks if there is a given file present in the given directories.
/// Returns the complete path to the file if it exists.
pub fn file_exists<'a>(file: &'a str, dirs: Vec<&'a str>) -> Option<String> {
    for dir in dirs {
        let path = format!("{}/{}", dir, file);
        if Path::new(&path).exists() {
            return Some(path);
        }
    }

    None
}

/// Checks if a file exists locally or as an executable in the PATH.
/// Returns the complete path to the file.
pub fn get_executable<'a>(file: &'a str) -> Option<String> {
    // if Path::new(&file).exists() {
    //     return Some(file.to_string());
    // }

    let path = env::var("PATH").unwrap_or_else(|_| "".to_string());
    let dirs = path.split(":").collect::<Vec<&str>>();
    file_exists(file, dirs)
}
