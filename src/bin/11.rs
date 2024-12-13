use std::collections::HashMap;

fn do_blink(x: u64) -> Vec<u64> {
    match x {
        0 => vec![1],
        n if n.to_string().len() % 2 == 0 => {
            let digits = n.to_string(); // TODO: can we avoid the .to_string() again?
            let mid = digits.len() / 2;
            let left: u64 = digits[..mid].parse().unwrap();
            let right: u64 = digits[mid..].parse().unwrap();
            vec![left, right]
        }
        _ => vec![x*2024],
    }
}

fn n_stones(input: &str, blinks: u8) -> u64 {

    let stones: Vec<u64> = input.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
    let mut stones_map: HashMap<u64, u64> = HashMap::new();
    for stone in &stones {
        let count = stones_map.entry(*stone).or_insert(0);
        *count += 1;
    }

    for _ in 0..blinks {
        // For each unique stone in this iteration (note the clone)
        for (stone, num_instances) in stones_map.clone().iter() {

            // reduce count of this stone by the no. of instances about to be transformed
            let count = stones_map.entry(*stone).or_insert(0);
            *count -= num_instances;
            if *count == 0 {
                stones_map.remove(&stone);
            }

            // increase count of newly created stones
            for new_stone in do_blink(*stone) {
                let count = stones_map.entry(new_stone).or_insert(0);
                *count += num_instances;
            }
        }
    }
    stones_map.values().sum()
}

fn solution_a(input: &str) -> u64 {
    n_stones(input, 25)
}

fn solution_b(input: &str) -> u64 {
    n_stones(input, 75)
}

fn main() {
    aoc2024::run("11", solution_a, solution_b)
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn it_works() {

        let input: &str = "125 17";
        let result = n_stones(input, 6);
        assert_eq!(result, 22);
    }
}