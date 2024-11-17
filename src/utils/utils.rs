use std::fs;
use std::path::Path;

pub fn read_file_data(relative_path: String) -> String {
    let test_path = Path::new(&relative_path);
    let test_data = fs::read_to_string(test_path).unwrap();

    test_data
}
