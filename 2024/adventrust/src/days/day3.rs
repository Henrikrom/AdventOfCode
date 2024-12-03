pub struct Day3 {
}

impl Day3 {
    pub fn part1(input: String) {
        let mut all_multiplications: Vec<(i32,i32)> = Vec::new();
        let input_chars: Vec<char> = input.chars().collect();

        for i in 0..input.len() {
            if input_chars[i] == 'm' {
                match Day3::is_real_mul(&input_chars, i) {
                    Some(result) => {
                        all_multiplications.push(result);
                    }
                    None => continue,
                }
            }
        }

        let mut sum = 0;
        for multiplication in all_multiplications  {
            sum += multiplication.0 * multiplication.1;
        }

        println!("Sum: {}", sum);
    }

    pub fn part2(input: String) {
        let mut is_do = true;
        
        let mut all_multiplications: Vec<(i32,i32)> = Vec::new();
        let input_chars: Vec<char> = input.chars().collect();

        for i in 0..input.len() {
            if Day3::is_do(&input_chars, i) {
                is_do = true;
            }

            if Day3::is_dont(&input_chars, i) {
                is_do = false;
            }

            if is_do {
                match Day3::is_real_mul(&input_chars, i) {
                    Some(result) => {
                        all_multiplications.push(result);
                    }
                    None => continue,
                }
            }
        }

        let mut sum = 0;
        for multiplication in all_multiplications  {
            sum += multiplication.0 * multiplication.1;
        }

        println!("Sum: {}", sum);
    }

    fn is_do(input: &Vec<char>, pos: usize) -> bool {
        if input[pos] != 'd' || input[pos+1] != 'o' || input[pos+2] != '(' || input[pos+3] != ')' {
            return false;
        }

        return true;
    }

    fn is_dont(input: &Vec<char>, pos: usize) -> bool {
        if input[pos] != 'd' ||
            input[pos+1] != 'o' || 
            input[pos+2] != 'n' || 
            input[pos+3] != '\'' || 
            input[pos+4] != 't' || 
            input[pos+5] != '(' || 
            input[pos+6] != ')' {
            return false;
        }

        return true;
    }

    fn is_real_mul(input: &Vec<char>, pos: usize) -> Option<(i32,i32)> {
        if input.len() < pos + 7 {
            return None;
        }

        if input[pos] != 'm' {
            return None;
        } 

        if input[pos + 1] != 'u' || input[pos + 2] != 'l' || input[pos + 3] != '(' {
            return None; 
        } 

        let mut first_number = 0;
        let mut second_number = 0; 
        let mut continue_to_index: usize = 1;

        match Day3::get_number(input, pos + 4) {
            Some(result) => {
                first_number = result.0; 
                continue_to_index = result.1;
            }
            None => return None,        
        }
        
        if input[continue_to_index] == ',' {
            continue_to_index += 1;
        }
        else {
            return None;
        }

        match Day3::get_number(input, continue_to_index) {
            Some(result) => {
                second_number = result.0; 
                continue_to_index = result.1;
            }
            None => return None,        
        }

        if input[continue_to_index] == ')' {
            return Some((first_number, second_number));
        }
        else {
            return None;
        }
    }

    fn get_number(input: &Vec<char>, pos: usize) -> Option<(i32, usize)> {
        if !input[pos].is_numeric() {
            return None;
        }

        let mut number = String::new();
        let mut last_number_pos: usize = 0;
        
        for i in pos..input.len() {
            if input[i].is_numeric() {
                number.push(input[i]);
            }
            else {
                last_number_pos = i.try_into().unwrap();
                break;
            }
        }

        let number: i32 = number.parse().unwrap();
        return Some((number, last_number_pos));
    }
}
