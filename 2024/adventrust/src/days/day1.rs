pub struct Day1 {
    pub input: String,
    left_numbers: Vec<i32>,
    right_numbers: Vec<i32>,
}

impl Day1 {
    pub fn new(input: String) -> Day1 {
        Day1 {
            input: input,
            left_numbers: Vec::new(),
            right_numbers: Vec::new(),
        }
    }

    pub fn part1(&mut self) {
        let lines_in_input: Vec<&str> = self.input.lines().collect();

        let mut left_numbers: Vec<i32> = Vec::new();
        let mut right_numbers: Vec<i32> = Vec::new();

        for line in lines_in_input {
            let mut first_number: String = String::new();
            let mut second_number: String = String::new();

            let mut has_passed_first_number: bool = false;

            for character in line.chars() {
                if character.is_numeric() && !has_passed_first_number {
                    first_number.push(character);
                } else if character.is_whitespace() {
                    has_passed_first_number = true;
                } else if character.is_numeric() {
                    second_number.push(character);
                }
            }

            let first_number: i32 = first_number
                .parse::<i32>()
                .expect("Could not parse string into i32");
            let second_number: i32 = second_number
                .parse::<i32>()
                .expect("Could not parse string into i32");

            left_numbers.push(first_number);
            right_numbers.push(second_number);
        }

        left_numbers.sort();
        right_numbers.sort();

        let mut diffs: Vec<i32> = Vec::new();
        for i in 0..left_numbers.len() {
            let left_number = left_numbers[i];
            let right_number = right_numbers[i];

            let diff = (left_number - right_number).abs();
            diffs.push(diff);
        }

        let sum_of_diffs: i32 = diffs.iter().sum();
        println!("Sum of diffs: {}", sum_of_diffs);

        self.left_numbers = left_numbers;
        self.right_numbers = right_numbers;
    }

    pub fn part2(&self) {
        let mut count: usize = 0;
        for i in 0..self.left_numbers.len() {
            let number_of_occurences = self
                .right_numbers
                .iter()
                .filter(|&&x| x == self.left_numbers[i])
                .count();

            let left_number: usize = self.left_numbers[i].try_into().unwrap();
            count = count + number_of_occurences * left_number;
        }

        println!("Similarity score: {}", count);
    }
}
