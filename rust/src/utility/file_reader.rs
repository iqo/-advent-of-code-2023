use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_lines (filename: &str) -> io::Result<Vec<String>> {
    let file = File.open(filename)?;
    let reader = BufReader::new(file);
    read.lines().collect()
}