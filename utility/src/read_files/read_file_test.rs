use crate::read_files::read_file_lines;
use std::io::{self, Write};
use tempfile::NamedTempFile;

#[test]
fn test_read_file_lines() ->  io::Result<()> {
    let mut temp_file: NamedTempFile =  NamedTempFile::new()?;

    writeln!(temp_file, "Hello")?;
    writeln!(temp_file, "World")?;
    let file_path: &str = temp_file.path().to_str().unwrap();
    let lines: Vec<String> = read_file_lines(file_path)?;
    assert_eq!(lines, vec!["Hello", "World"]);
    Ok(())
}

#[test]
fn test_read_empty_file() -> io::Result<()> {
    let temp_file: NamedTempFile = NamedTempFile::new()?;
    let file_path: &str = temp_file.path().to_str().unwrap();
    let lines: Vec<String> = read_file_lines(file_path)?;
    assert!(lines.is_empty());
    Ok(())
}

#[test]
fn test_read_nonexistent_file() -> io::Result<()>  {
    let result: Result<Vec<String>, io::Error> = read_file_lines("nonexistent_file.txt");
    assert!(result.is_err());
    Ok(())
}