use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_file_lines (filename: &str) -> io::Result<Vec<String>> {
    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);
    reader.lines().collect()
}
