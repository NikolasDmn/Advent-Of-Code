use itertools::Itertools;
use rayon::prelude::*;
const EXAMPLE_ANSWER: usize = 4;

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
fn is_valid(nums: &Vec<isize>) -> bool {
    let det = nums[0] - nums[1];
    let mut multiplier = 1;
    if det == 0 {
        return false;
    }
    if det < 0 {
        multiplier = -1;
    }
    return nums
        .into_iter()
        .tuple_windows()
        .map(|(x, y)| multiplier * (x - y))
        .all(|step| step >= 1 && step <= 3);
}
fn solve(input: &str) -> usize {
    // Brute force, probably a bad idea but at least there is paralellism
    input
        .par_lines()
        .map(|line| {
            line.trim()
                .split(" ")
                .map(|n| n.parse::<isize>().unwrap())
                .collect()
        })
        .filter(|nums: &Vec<isize>| {
            (0..nums.len()).into_par_iter().any(|i| {
                let mut n = nums.clone();
                n.remove(i);
                return is_valid(&n);
            })
        })
        .count()
}
