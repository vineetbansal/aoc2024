use std::fs;

fn is_correct(constraint: (i16, i16), this: i16, others: &[i16]) -> bool {
    if this == constraint.1 {
        for other in others {
            if *other == constraint.0 {
                return false
            }
        }
    }
    true
}

fn whatever(input: &str) -> i16 {
    let mut constraints: Vec<(i16, i16)> = vec![];
    let mut result: i16 = 0;

    let mut in_block0: bool = true;
    for line in input.lines() {
        if line == "" {
            in_block0 = false;
            continue;
        }

        if in_block0 {
            let c: Vec<i16> = line.split("|").map(|part| part.parse::<i16>().unwrap()).collect();
            constraints.push((c[0], c[1]));
        } else {
            let update: Vec<i16> = line.split(",").map(|s| s.parse::<i16>().unwrap()).collect();
            let mut all_correct: bool = true;
            for i in 0..update.len() {
                for constraint in &constraints {
                    if !is_correct(*constraint, update[i], &update[i..]) {
                        all_correct = false;
                        break;
                    }
                }
            }
            if all_correct {
                result = result + update[update.len() / 2];
            }
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input5_1.txt").unwrap();
    let result = whatever(input.as_str());
    print!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let input: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = whatever(input);
        assert_eq!(result, 143);
    }
}