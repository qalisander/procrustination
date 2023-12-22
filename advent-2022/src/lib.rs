use std::fs;
use std::path::Path;

pub fn get_input_str(file: &str) -> String {
    let path = Path::new(file).parent().unwrap().join("input");
    fs::read_to_string(path).unwrap()
}
