mod days;

use std::fs;
use days::day1::Day1;

fn main() {
    solve_day1();
}

fn solve_day1() {
    let day1_input_path: &str = "src/inputs/day1";

    let day1_input: String = get_input_string(day1_input_path); 
    let mut day1: Day1 = Day1::new(day1_input);
    day1.part1();
    day1.part2();
}

fn get_input_string(file_path: &str) -> String {
    let file_content: Result<String, std::io::Error> = fs::read_to_string(file_path);
    
    match file_content {
        Ok(content) => {
            return content;
        }
        Err(_) => {
            println!("Failed to read file with path {file_path}");
            return String::new();
        } 
    }     
}