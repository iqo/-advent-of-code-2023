use std::path::Path;
use std::iter::zip;
use utility::read_file::read_input_file;
    fn main() -> std::io::Result<()> {
        let path = Path::new("day-01/data/").join("input.txt");
        let file_input = read_input_file(path.to_str().unwrap());
        let result: i32 = total_distance(&file_input);
        print!("Total distance: {}", result);
        Ok(())
    }

     pub fn total_distance(input: &str) -> i32 {
        let mut left_list: Vec<i32> = Vec::new();
        let mut rigth_list: Vec<i32> = Vec::new();
        for line in input.lines() {
            let mut split_data = line.split_whitespace();
            left_list.push(split_data.next().unwrap().parse::<i32>().unwrap());
            rigth_list.push(split_data.next().unwrap().parse::<i32>().unwrap());
        }
        left_list.sort();
        rigth_list.sort();
        let calcultade_distance: i32 = zip(left_list, rigth_list).map(|(a, b)|(a - b).abs()).sum();
        calcultade_distance
    } 

    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> std::io::Result<()> {
         let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        let result: i32 = total_distance(&input);
        assert_eq!(11, result);
        Ok(()) 
    }
}