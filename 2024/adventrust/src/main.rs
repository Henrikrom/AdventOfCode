mod days;

use days::day1::Day1;
use days::day2::Day2;
use days::day3::Day3;
use std::fs;

fn main() {
    //solve_day1();
    //solve_day2();
    solve_day3();
}

fn solve_day3() {
    let day3_input_path: &str = "src/inputs/day3";

    let day3_input: String = get_input_string(day3_input_path);
    Day3::part1(day3_input.clone());
    Day3::part2(day3_input);
}

fn solve_day2() {
    let day2_input_path: &str = "src/inputs/day2";

    let day2_input: String = get_input_string(day2_input_path);
    let input = Day2::parse(day2_input);
    Day2::part1(&input);
    Day2::part2(&input);
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
