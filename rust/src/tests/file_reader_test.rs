use std::io::{self, Write};
use tempfile::NamedTempFile;
use rust::read_file_lines;

#[test]
fn test_read_file_lines() -> io::Result<()> {
    let mut temp_file =  NamedTempFile::new()?;

    writeln!(temp_file, "Hello")?;
    writeln!(temp_file, "World")?;
    let file_path = temp_file.path().to_str().unwrap();
    let line = read_file_lines(file_path)?;
    assert_eq!(lines, vec!["Hello", "Woddrld"]);
}