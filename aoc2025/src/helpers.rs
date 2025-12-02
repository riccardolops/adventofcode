use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path.display()))
}

