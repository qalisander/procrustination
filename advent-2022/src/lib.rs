use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_file_str(file: &str) -> String {
//    let current_dir = env::current_dir().unwrap();
    let mut test_file = File::open(Path::new("src").join(file)).unwrap();
    let mut test_data = String::new();
    test_file.read_to_string(&mut test_data).unwrap();
    test_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
