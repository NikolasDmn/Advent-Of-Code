use rayon::prelude::*;
use regex::Regex;
const EXAMPLE_ANSWER: usize = 161;

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
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let re_num = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|h| h.as_str())
        .map(move |m| {
            re_num
                .find_iter(m)
                .map(|n| n.as_str().parse::<usize>().unwrap())
                .product::<usize>()
        })
        .sum()
}
