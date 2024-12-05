use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

pub fn read_file_lines (filename: &str) -> io::Result<Vec<String>> {
    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);
    reader.lines().collect()
}

pub fn read_file_lines_no_buffer (filename: &str) -> String{
    let data = fs::read_to_string(filename).expect("uanble to read file");
    data
}
pub fn read_input_file() -> String {
    fs::read_to_string("src/input.txt").unwrap()
}