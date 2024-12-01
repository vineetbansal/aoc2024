use std::fs;


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


fn main() {
    let allow_bad_level: bool = true;  // problem a or b
    let document = fs::read_to_string("input2_1.txt").unwrap();
    let mut safe_reports: u32 = 0;
    for line in document.lines() {
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

    print!("{:?}", safe_reports);
}
