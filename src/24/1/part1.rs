use rayon::{prelude::*, vec};
const EXAMPLE_ANSWER: usize = 11;

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
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            l.split("   ")
                .map(|n| n.trim().parse::<i32>().expect(&format!("{}", n)))
                .collect::<Vec<i32>>()
        })
        .map(|nums| (nums[0], nums[1]))
        .unzip();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>() as usize
}
