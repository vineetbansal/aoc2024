fn create_arrays(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut arr1:Vec<u32> = Vec::new();
    let mut arr2:Vec<u32> = Vec::new();
    for line in input.lines() {
        let splits: Vec<u32> = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        arr1.push(splits[0]);
        arr2.push(splits[1]);
    }
    arr1.sort();
    arr2.sort();

    (arr1, arr2)
}

fn solution_a(input: &str) -> u32 {
    let (a, b) = create_arrays(input);
    a.iter().zip(b.iter()).map(|(x, y)| u32::abs_diff(*x, *y)).sum()
}

fn solution_b(input: &str) -> u32 {
    let (a, b) = create_arrays(input);
    a.iter().filter(|x| b.contains(x)).sum()
}

fn main() {
    aoc2024::run("01", solution_a, solution_b);
}