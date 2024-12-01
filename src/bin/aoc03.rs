use std::fs;
use regex::Regex;


fn mult_result(s: &str) -> u32 {
    let re = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();
    let pairs: Vec<(u32, u32)> = re.captures_iter(s).map(|caps| {
        let first = caps.name("first").unwrap().as_str().parse::<u32>().unwrap();
        let second = caps.name("second").unwrap().as_str().parse::<u32>().unwrap();
        (first, second)
    }).collect();

    let sum: u32 = pairs.iter().map(|&(x, y)| x * y).sum();
    sum
}


fn preprocess(s: &str) -> String {
    let re = Regex::new(r"^(.*?)don't\(\)|do\(\)(.*?)don't\(\)|do\(\)(.*)").unwrap();

    let mut before_first_dont = None;
    let mut last_do_to_end = None;
    let mut between_do_dont = Vec::new();

    for cap in re.captures_iter(s) {
        if let Some(before) = cap.get(1) {
            before_first_dont = Some(before.as_str());
        }
        if let Some(between) = cap.get(2) {
            between_do_dont.push(between.as_str());
        }
        if let Some(after) = cap.get(3) {
            last_do_to_end = Some(after.as_str());
        }
    }

    let mut result = String::new();
    if let Some(before) = before_first_dont {
        result.push_str(&before);
    }
    for between in between_do_dont {
        result.push_str(&between);
    }
    if let Some(after) = last_do_to_end {
        result.push_str(&after);
    }
    result
}

fn main() {
    let document = fs::read_to_string("input3_1.txt").unwrap();
    // let result = mult_result(&document);
    let result = mult_result(preprocess(&document).as_str());
    print!("{:?}", result);

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works1() {
        let input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = mult_result(input);
        assert_eq!(result, 161);
    }

    #[test]
    fn it_works2() {
        let input: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let preprocessed: String = preprocess(input);
        let result = mult_result(preprocessed.as_str());
        assert_eq!(result, 48);
    }
}