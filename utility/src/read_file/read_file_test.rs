use crate::read_file::read_input_file;
use std::io::{self, Write};
use tempfile::NamedTempFile;

#[test]
fn test_read_file_lines2() ->  io::Result<()> {
    let mut temp_file: NamedTempFile =  NamedTempFile::new()?;
    writeln!(temp_file, "Hello World")?;
    let file_path: &str = temp_file.path().to_str().unwrap();
    let lines: String = read_input_file(file_path);
    assert_eq!(lines, "Hello World\n");
    Ok(())
}