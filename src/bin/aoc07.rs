use std::fs;


// #[f64] is a slice instead of Vec<f64> which is a full vector
fn try_op(numbers: &[f64], desired_result: f64, allow_concat: bool) -> (Vec<&str>, bool) {
    if numbers.len() == 1 {
        if numbers[0] == desired_result {
            return (vec![], true);
        } else {
            return (vec![], false);
        }
    }

    let last_number: f64 = *numbers.last().unwrap();

    // Really should be doing this iterating over an Enum, rather than independent blocks
    // Try *
    let (subvec, found) = try_op(&numbers[..numbers.len()-1], desired_result / last_number, allow_concat);
    if found {
        return ([subvec, vec!["*"]].concat(), true);
    }

    // Try +
    let (subvec, found) = try_op(&numbers[..numbers.len()-1], desired_result - last_number, allow_concat);
    if found {
        return ([subvec, vec!["+"]].concat(), true);
    }

    // Try ||
    if allow_concat {
        // For every possible first-n digits of desired_result, see if can match the rest
        let desired_result_str = desired_result.to_string();
        for i in 0..desired_result_str.len() {
            let prefix: i64 = desired_result_str[0..i].to_string().parse::<i64>().unwrap_or(0);
            let suffix: i64 = desired_result_str[i..].to_string().parse::<i64>().unwrap_or(0);
            if numbers[0] as i64 == prefix {
                let (subvec, found) = try_op(&numbers[1..], suffix as f64, allow_concat);
                if found {
                    return ([subvec, vec!["||"]].concat(), true);
                }
            }
            if numbers[numbers.len()-1] as i64 == suffix {
                let (subvec, found) = try_op(&numbers[..numbers.len()-1], prefix as f64, allow_concat);
                if found {
                    return ([subvec, vec!["||"]].concat(), true);
                }
            }
        }
    }
    return (vec![], false);
}

fn calibrate(string: &str, allow_concat: bool) -> i64 {
    let mut final_result: f64 = 0.;
    for line in string.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        assert_eq!(parts.len(), 2);
        let result = parts[0].parse::<f64>().unwrap();
        let numbers: Vec<f64> = parts[1].trim().split(" ").map(|s| s.parse::<f64>().unwrap()).collect();
        let (_, found) = try_op(&numbers, result, allow_concat);
        if found {
            final_result += result;
        }
    }
    final_result as i64
}

fn main() {
    let string = fs::read_to_string("input7_1.txt").unwrap();
    let result = calibrate(string.as_str(), true);
    print!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_a() {
        let input: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = calibrate(input, false);
        assert_eq!(result, 3749);
    }

    #[test]
    fn it_works_b() {
        let input: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = calibrate(input, true);
        assert_eq!(result, 11387);
    }
}
