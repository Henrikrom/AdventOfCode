pub struct Day4 {
}

impl Day4 {
    pub fn part1(input: &String) {
        let mut count = 0;

        let input = Day4::input_to_vector(&input);
        for y in 1..input.len() - 1{
            for x in 1..input.len() - 1 {
                count += Day4::xmas_count(&input, (x,y));        
            }
        }

        println!("XMAS count: {}", count);
    }

    pub fn part2(input: &String) {
         let mut count = 0;

        let input = Day4::input_to_vector(&input);
        for y in 1..input.len() - 1{
            for x in 1..input.len() - 1 {
                count += Day4::x_mas_count(&input, (x,y));        
            }
        }

        println!("X-MAS count: {}", count);
    }

    fn x_mas_count(input: &Vec<Vec<char>>, pos: (usize, usize)) -> i32 {
        let xpos: i32 = pos.0.try_into().unwrap();
        let ypos: i32 = pos.1.try_into().unwrap();

        let mut up_left = 'O';
        let mut up_right = 'O';
        let mut down_left = 'O';
        let mut down_right = 'O';
        if input[pos.1][pos.0] == 'A' {
            let xindex = xpos - 1;
            let yindex = ypos - 1;
            let xindex_usize:usize = xindex.try_into().unwrap();
            let yindex_usize:usize = yindex.try_into().unwrap();

            if input[yindex_usize][xindex_usize] == 'M' {
                up_left = 'M'
            }
            if input[yindex_usize][xindex_usize] == 'S' {
                up_left = 'S'
            }

            let xindex = xpos - 1;
            let yindex = ypos + 1;
            let xindex_usize:usize = xindex.try_into().unwrap();
            let yindex_usize:usize = yindex.try_into().unwrap();

            if input[yindex_usize][xindex_usize] == 'M' {
                down_left = 'M'
            }
            if input[yindex_usize][xindex_usize] == 'S' {
                down_left = 'S'
            }

            let xindex = xpos + 1;
            let yindex = ypos + 1;
            let xindex_usize:usize = xindex.try_into().unwrap();
            let yindex_usize:usize = yindex.try_into().unwrap();

            if input[yindex_usize][xindex_usize] == 'M' {
                down_right = 'M'
            }
            if input[yindex_usize][xindex_usize] == 'S' {
                down_right = 'S'
            }

            let xindex = xpos + 1;
            let yindex = ypos - 1;
            let xindex_usize:usize = xindex.try_into().unwrap();
            let yindex_usize:usize = yindex.try_into().unwrap();

            if input[yindex_usize][xindex_usize] == 'M' {
                up_right = 'M'
            }
            if input[yindex_usize][xindex_usize] == 'S' {
                up_right = 'S'
            }
        }

        let mut count = 0;
    
        if up_left == 'M' && up_right == 'M' && down_left == 'S' && down_right == 'S' {
            return 1;
        }
        if up_left == 'S' && up_right == 'S' && down_left == 'M' && down_right == 'M' {
            return 1;
        }
        if up_left == 'S' && up_right == 'M' && down_left == 'S' && down_right == 'M' {
            return 1;
        }
        if up_left == 'M' && up_right == 'S' && down_left == 'M' && down_right == 'S' {
            return 1;
        }

        return count;
    }

    fn xmas_count(input: &Vec<Vec<char>>, pos: (usize, usize)) -> i32 {
        let xpos: i32 = pos.0.try_into().unwrap();
        let ypos: i32 = pos.1.try_into().unwrap();

        if input[pos.1][pos.0] != 'X' {
            return 0;
        }

        let mut count = 0;

        let directions: Vec<(i32, i32)> = Day4::get_directions(); 
        for direction in directions {
            let xindex = xpos + direction.0;
            let yindex = ypos + direction.1;
            let xindex_usize:usize = xindex.try_into().unwrap();
            let yindex_usize:usize = yindex.try_into().unwrap();

            if input[yindex_usize][xindex_usize] != 'M' {
                continue;
            }

            let xindex = xindex + direction.0;
            let yindex = yindex + direction.1;
            let xindex_usize:usize = xindex.try_into().unwrap();
            let yindex_usize:usize = yindex.try_into().unwrap();

            if input[yindex_usize][xindex_usize] != 'A' {
                continue;
            }

            let xindex = xindex + direction.0;
            let yindex = yindex + direction.1;
            let xindex_usize:usize = xindex.try_into().unwrap();
            let yindex_usize:usize = yindex.try_into().unwrap();

            if input[yindex_usize][xindex_usize] != 'S' {
                continue;
            }

            count += 1;
        }

        count
    }


    fn input_to_vector(input: &String) -> Vec<Vec<char>> {
        let mut input_as_vector: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            let mut line_as_vector: Vec<char> = Vec::new();
            
            for character in line.chars() {
                line_as_vector.push(character);
            }

            input_as_vector.push(line_as_vector);
        }

        return input_as_vector;
    }

    fn get_directions() -> Vec<(i32, i32)> {
        vec![
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
    }

}
