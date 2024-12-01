use rayon::prelude::*;
const EXAMPLE_ANSWER: usize = 24000;

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
    println!("Algorithm Finished.\nOutput: {}", answer);
}

fn solve(input: &str) -> usize {
    input
        .split("\r\n\r\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.trim().parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}
