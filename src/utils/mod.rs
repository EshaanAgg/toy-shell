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
