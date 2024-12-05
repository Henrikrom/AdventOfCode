use std::collections::HashMap;
pub struct Day5 {
}

impl Day5 {
    pub fn parse_rules(input: &String) -> Vec<Vec<i32>> {
       let mut rules: Vec<Vec<i32>> = Vec::new();

       for line in input.lines() {
        if line.contains('|') {
            let rule: Vec<i32> = line.split('|').map(|c| c.parse::<i32>().unwrap()).collect();
            rules.push(rule);
        }
       } 
     
       return rules;
    }

    pub fn parse_pages_to_update(input: &String) -> Vec<Vec<i32>> {
        let mut pages: Vec<Vec<i32>> = Vec::new();

        for line in input.lines() {
            if !line.contains('|') {
                let page: Vec<i32> = line.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
                pages.push(page);
            }
        } 

        return pages
    }

    pub fn part1(rules: &Vec<Vec<i32>>, pages: &Vec<Vec<i32>>) {
        let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for rule in rules {
            rule_map.entry(rule[0]).or_insert_with(Vec::new).push(rule[1]);
        }

        let mut allowed_pages: Vec<&Vec<i32>> = Vec::new();
        for page in pages {
            if Day5::is_page_allowed(&rule_map, &page) {
                allowed_pages.push(page);
            }
        }

        let mut sum = 0;
        for page in allowed_pages {
            let value_to_add = page[(page.len()-1)/2];
            sum += value_to_add;
        }

        println!("Sum: {}", sum);
    }

    pub fn part2(rules: &Vec<Vec<i32>>, pages: &Vec<Vec<i32>>) {
        let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for rule in rules {
            rule_map.entry(rule[0]).or_insert_with(Vec::new).push(rule[1]);
        }

        let mut not_allowed_pages: Vec<&Vec<i32>> = Vec::new();
        for page in pages {
            if !Day5::is_page_allowed(&rule_map, &page) {
                not_allowed_pages.push(page);
            }
        }

        let mut sorted_pages: Vec<Vec<i32>> = Vec::new();
        for page in not_allowed_pages {
            let sorted_page = Day5::sort_page(&rule_map, page);
            sorted_pages.push(sorted_page);
        }


        let mut sum = 0;
        for page in sorted_pages {
            let value_to_add = page[(page.len()-1)/2];
            sum += value_to_add;
        }

        println!("Sum part 2: {}", sum);
    }

    pub fn sort_page(rules: &HashMap<i32, Vec<i32>>, page: &Vec<i32>) -> Vec<i32> {
        let mut sorted_page: Vec<i32> = page.clone();
        loop {
            if Day5::is_page_allowed(rules, &sorted_page) {
                break;
            }

            let mut should_continue = false;
            for i in 0..page.len() {
                if should_continue {
                    continue;
                }

                let page_value_base = sorted_page[i];
                for j in i..page.len() {
                    if i == j {
                        continue;
                    }

                    let page_value = sorted_page[j];
                    if !rules.contains_key(&page_value) {
                        continue;
                    }

                    let values_that_are_not_allowed = rules.get(&page_value).unwrap();
                    if values_that_are_not_allowed.contains(&page_value_base) {
                        sorted_page.swap(j, j - 1);
                        should_continue = true;
                        break;
                    }  
                }
        }
           
        }

        return sorted_page;
    }

    pub fn is_page_allowed(rules: &HashMap<i32, Vec<i32>>, page: &Vec<i32>) -> bool {
        let mut page_is_allowed = true;
        for i in 0..page.len() {
            let page_value_base = page[i];
            for j in i..page.len() {
                if i == j {
                    continue;
                }

                let page_value = page[j];
                if !rules.contains_key(&page_value) {
                    continue;
                }

                let values_that_are_not_allowed = rules.get(&page_value).unwrap();
                for x in values_that_are_not_allowed {
                }
                if values_that_are_not_allowed.contains(&page_value_base) {
                    page_is_allowed = false;
                }  
            }
        }

        return page_is_allowed;
    }
}
