pub struct Day2 {
}

impl Day2 {
    pub fn parse(input: String) -> Vec<Vec<i32>> {
        let mut reports: Vec<Vec<i32>> = Vec::new();

        for line in input.lines() {
            let levels: Vec<i32> = line.split(" ").map(|c| c.parse::<i32>().unwrap()).collect();
            reports.push(levels);
        }

        return reports;
    }

    pub fn part1(input: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut safe_count = 0;
        
        let mut unsafe_reports = Vec::new();
        for report in input {
            if Day2::report_is_safe(report) {
                safe_count += 1;
            }
            else {
                unsafe_reports.push(report.clone());
            }
        }

        println!("Safe report count: {}", safe_count);
        return unsafe_reports;
    }

    pub fn part2(input: &Vec<Vec<i32>>) {
        let unsafe_reports = Day2::part1(&input);
        let mut safe_count = 0;

        for report in &unsafe_reports {
            let mut report_tmp = report.clone();
            for i in 0..report.len() {
                report_tmp.remove(i);
                if Day2::report_is_safe(&report_tmp) {
                    safe_count += 1;
                    break; 
                }

                report_tmp = report.clone();
            }
        }

        let safe_reports = input.len() - unsafe_reports.len() + safe_count;
        println!("Safe reports part2: {}", safe_reports);
    }

    fn report_is_safe(report: &Vec<i32>) -> bool {
        let mut direction = 0;

        let mut level_is_safe = true;
        for i in 1..report.len() {
            if report[i] == report[i-1] {
                level_is_safe = false;
                break;
            }

            if direction == 0 {
                if report[i] > report[i-1] {
                    direction = 1;
                }
                else {
                    direction = -1;
                }
            }

            if direction == 1 {
                if report[i] < report[i-1] {
                    level_is_safe = false;
                    break;
                }
            }
            else {
                if report[i] > report[i-1] {
                    level_is_safe = false;
                    break;
                }
            }

            let diff = (report[i] - report[i-1]).abs();
            if diff > 3 {
                level_is_safe = false;
                break; 
            }
        }

        return level_is_safe;
    }

}
