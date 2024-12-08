
use itertools::Itertools;

pub struct Day7 {
}


#[derive(Default)]
pub struct Equations {
    pub test_value: i32,
    pub equation_numbers: Vec<i32>
}

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

    pub fn part1(input: Vec<Equations>) {


    }

    fn equation_is_possible(equation: Equations) {

    }

    fn get_all_perms_of_operator(operators_count: i32) -> Vec<Vec<Operators>> {
        let mut permuations: Vec<Vec<Operators>> = Vec::new();

        let operators= vec![Operators::Pluss, Operators::Multiply];
        let combinations: Vec<Vec<Operators>> = operators
            .iter()
            .combinations_with_replacement(operators_count.try_into().unwrap())
            .flat_map(|c| c.into_iter().permutations(operators_count.try_into().unwrap()))
            .collect();

        return permuations;
    }
}