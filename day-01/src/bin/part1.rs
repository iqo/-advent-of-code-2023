use std::path::Path;
use utility::read_file::read_file_lines;

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
        total_distance(&lines);
        Ok(())
    }
    fn total_distance(input: &Vec<String>) {
        let mut left_list: Vec<String> = Vec::new();
        let mut rigth_list: Vec<String> = Vec::new();
        for line in input {
            let mut split_data = line.split_whitespace();
            left_list.push(split_data.next().unwrap().parse::<i32>().unwrap().to_string());
            rigth_list.push(split_data.next().unwrap().parse::<i32>().unwrap().to_string());
            left_list.sort();
            rigth_list.sort();
        println!("left: {:?}, rigth: {:?}", left_list, rigth_list);
        }
    }