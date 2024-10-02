use std::io::{self, Write};
use tempfile::NamedTempFile;
use rust::utility::read_file_lines;

#[test]
fn test_read_file_lines() ->  io::Result<()> {
    let mut temp_file =  NamedTempFile::new()?;

    writeln!(temp_file, "Hello")?;
    writeln!(temp_file, "World")?;
    let file_path = temp_file.path().to_str().unwrap();
    let lines = read_file_lines(file_path)?;
    assert_eq!(lines, vec!["Hello", "World"]);
    Ok(())
}

#[test]
fn test_read_empty_file() -> io::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let file_path = temp_file.path().to_str().unwrap();
    let lines = read_file_lines(file_path)?;
    assert!(lines.is_empty());
    Ok(())
}

#[test]
fn test_read_nonexistent_file() -> io::Result<()>  {
    let result = read_file_lines("nonexistent_file.txt");
    assert!(result.is_err());
    Ok(())
}