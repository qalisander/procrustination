use std::path::Path;
use std::{fs};

pub fn get_input_str(file: &str) -> String {
    let path = Path::new(file).parent().unwrap().join("input");
    fs::read_to_string(path).unwrap()
}