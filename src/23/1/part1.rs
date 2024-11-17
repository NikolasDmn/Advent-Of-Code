use rayon::prelude::*;
const EXAMPLE_ANSWER: usize = 142;

fn main() {
    let sample = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let sample_answer = solve(sample);
    if sample_answer != EXAMPLE_ANSWER {
        eprintln!(
            "Algorithm wrong from example:\nExpected: {}\nProvided: {}",
            EXAMPLE_ANSWER, sample_answer
        );
        return;
    }
    println!("Starting Algorithm:\n - Sample passed [CHECK]\n - Running Algorithm...");
    let answer = solve(input);
    println!("Algorithm Finished...\nOutput: {}", answer);
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<&str>>()
        .par_iter()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first) as usize;
            first as usize * 10 + last
        })
        .sum()
}
