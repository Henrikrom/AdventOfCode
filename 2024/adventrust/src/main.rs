mod days;

use days::day1::Day1;
use days::day2::Day2;
use days::day3::Day3;
use days::day4::Day4;
use days::day5::Day5;
use std::fs;

fn main() {
    //solve_day1();
    //solve_day2();
    //solve_day3();
    //solve_day4();
    solve_day5();
}

fn solve_day5() {
    let day5_input_path: &str = "src/inputs/day5";

    let day5_input: String = get_input_string(day5_input_path);
    let rules = Day5::parse_rules(&day5_input);
    let pages = Day5::parse_pages_to_update(&day5_input);
    Day5::part1(&rules, &pages);
    Day5::part2(&rules, &pages);
}

fn solve_day4() {
    let day4_input_path: &str = "src/inputs/day4";

    let day4_input: String = get_input_string(day4_input_path);
    //Day4::part1(&day4_input);
    Day4::part2(&day4_input);
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
