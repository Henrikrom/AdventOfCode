mod days;

use days::day1::Day1;
use days::day2::Day2;
use days::day3::Day3;
use days::day4::Day4;
use days::day5::Day5;
use days::day6::Day6;
use days::day7::Day7;
use std::fs;

fn main() {
    //solve_day1();
    //solve_day2();
    //solve_day3();
    //solve_day4();
    //solve_day5();
    //solve_day6();
    solve_day7();
}

fn solve_day7() {
    let day7_input_path: &str = "src/inputs/day7_test";

    let day7_input: String = get_input_string(day7_input_path);
    let parsed_input = Day7::parse(&day7_input);
    //let parsed_map = Day7::parse_map(&day7_input);
    //Day7::part1(&mut parsed_map.clone());
}

fn solve_day6() {
    let day6_input_path: &str = "src/inputs/day6";

    let day6_input: String = get_input_string(day6_input_path);
    let parsed_map = Day6::parse_map(&day6_input);
    //Day6::part1(&mut parsed_map.clone());
    Day6::part2(&mut parsed_map.clone());
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
