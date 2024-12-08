
use itertools::{equal, Itertools};

pub struct Day7 {
}


#[derive(Default)]
pub struct Equations {
    pub test_value: i32,
    pub equation_numbers: Vec<i32>
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Operators {
    Pluss,
    Multiply
}

impl Day7 {
    pub fn parse(input: &String) -> Vec<Equations> {
        let mut equations: Vec<Equations> = Vec::new();

        for line in input.lines() {
            let mut equation_split = line.split(":");
            let test_value: i32 = equation_split.next().unwrap().trim().parse().unwrap();
            let equation_numbers: Vec<i32> = equation_split.next().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect(); 
            let equation = Equations {
                test_value: test_value,
                equation_numbers: equation_numbers
            };

            equations.push(equation);
        }

        return equations;
    }

    pub fn part1(input: &Vec<Equations>) {

        let mut sum = 0;
        for equation in input {
            if Day7::equation_is_possible(equation) {
                sum += equation.test_value;
            }
        }

        println!("Count: {}", sum);
    }

    fn equation_is_possible(equation: &Equations) -> bool {
        let number_of_operators: i32 = (equation.equation_numbers.len() - 1).try_into().unwrap();
        let operator_permutations = Day7::get_all_perms_of_operator(number_of_operators);

        for permutation in operator_permutations {
            let mut sum = equation.equation_numbers[0];
            for j in 0..equation.equation_numbers.len() - 1 {
                match permutation[j] {
                    Operators::Pluss => sum = sum + equation.equation_numbers[j+1],
                    Operators::Multiply => sum = sum * equation.equation_numbers[j+1]
                }
            }

            if sum == equation.test_value {
                return true;
            }
        }

        return false;
    }

    fn get_all_perms_of_operator(operators_count: i32) -> Vec<Vec<Operators>> {
        let mut permuations: Vec<Vec<Operators>> = Vec::new();

        let operators= vec![Operators::Pluss, Operators::Multiply];
        let combinations_with_replacement = operators
            .iter()
            .combinations_with_replacement(operators_count.try_into().unwrap());


        for combo in combinations_with_replacement {
            let combo_vec: Vec<Operators> = combo.into_iter().cloned().collect();

            let permutations = combo_vec.into_iter().permutations(operators_count.try_into().unwrap());
            for permutation     in permutations {
                permuations.push(permutation);
            }
        }

        return permuations;
    }
}