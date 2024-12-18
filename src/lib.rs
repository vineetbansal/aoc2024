/*!
Adapted from
https://raw.githubusercontent.com/henryiii/aoc2024/refs/heads/main/src/problems.rs

Dumbed down to aid my own incremental understanding of Rust.
 */

use std::time::Instant;
use std::fmt::Display;

pub fn run<F1, F2, T>(name: &str, solution_a: F1, solution_b: F2)
where
    F1: Fn(&str) -> T,
    F2: Fn(&str) -> T,
    T: Display
{
    let now = Instant::now();
    let input = std::fs::read_to_string(format!("data/{name}.txt")).unwrap();

    let sol_a_time = Instant::now();
    let solution_a = solution_a(&input);
    let sol_a_time = sol_a_time.elapsed().as_secs_f64() * 1000.0;
    println!("Day {name} Solution A: {solution_a} ({sol_a_time:.3}ms)");

    let sol_b_time = Instant::now();
    let solution_b = solution_b(&input);
    let sol_b_time = sol_b_time.elapsed().as_secs_f64() * 1000.0;
    println!("Day {name} Solution B: {solution_b} ({sol_b_time:.3}ms)");

    let time_taken = now.elapsed().as_secs_f64() * 1000.0;

    println!("Day {name} Time taken: {time_taken:.3}ms");
}