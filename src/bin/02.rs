fn is_safe(levels: &Vec<i32>) -> bool {
    let diff = levels[1] - levels[0];
    let direction: i32 = if diff > 0 {1} else {-1};
    for i in 1..levels.len() {
        if ![1, 2, 3].iter().any(|&x| levels[i] - levels[i-1] == direction * x) {
            return false;
        }
    }
    true
}

fn get_safe_reports(input: &str, allow_bad_level: bool) -> u32 {
    let mut safe_reports: u32 = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
        let mut safe: bool = is_safe(&levels);
        if !safe && allow_bad_level {
            for i in 0..levels.len() {
                let levels_sans_i: Vec<i32> = [&levels[0..i], &levels[i+1..]].concat();
                safe = is_safe(&levels_sans_i);
                if safe {
                    break;
                }
            }
        }
        safe_reports += safe as u32;
    }
    safe_reports
}

fn solution_a(input: &str) -> u32 {
    get_safe_reports(input, false)
}

fn solution_b(input: &str) -> u32 {
    get_safe_reports(input, true)
}

fn main() {
    aoc2024::run("02", solution_a, solution_b);
}


