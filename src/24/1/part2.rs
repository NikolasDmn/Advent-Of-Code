use std::collections::HashMap;

use rayon::prelude::*;
const EXAMPLE_ANSWER: usize = 31;

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
    let frequencies = right.into_iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    left.into_iter()
        .map(|num| num * frequencies.get(&num).unwrap_or(&0))
        .sum::<i32>() as usize
}
