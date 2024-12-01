use std::fs;
use std::path::Path;

pub fn read_file_data(relative_path: String) -> String {
    let path = Path::new(&relative_path);
    let data = fs::read_to_string(path).unwrap();

    data
}
