
// use std::env;
use std::path::Path;

 use utility::read_file::read_file::read_file_lines;
    //let path = Path::new("data").join("input.txt");
    //let lines = read_file_lines(path.to_str().unwrap());

    fn main() -> std::io::Result<()> {
    /* 
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    */
        let path = Path::new("day-01/data/").join("test_input.txt");
        let lines = read_file_lines(path.to_str().unwrap())?;
        total_distance(lines);
        Ok(())

        // let path1 = env::current_dir()?;
        // println!("The current directory is {}", path1.display());
        // Ok(())
    }
    fn total_distance(input: Vec<String>) {
        let mut left_list: Vec<String> = vec![];
        let mut rigth_list: Vec<String> = vec![];
        for line in input {
            let mut split_data = line.split_whitespace();
            todo!()
            //left_list.push(split_data.next().unwrap().parse::<32>().unwrap());
        }
    }