use std::fs::read_to_string;

pub fn read_input_file(filename: &str) -> String {
    read_to_string(filename).unwrap()
}