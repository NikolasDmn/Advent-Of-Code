use core::panic;

use rayon::prelude::*;
const EXAMPLE_ANSWER: usize = 8;

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

enum Balls {
    Red(usize),
    Blue(usize),
    Green(usize),
}
fn solve(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if line
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split("; ")
                .map(|set| {
                    set.split(", ").map(|ball| {
                        match ball.trim().split(" ").skip(1).next().unwrap() {
                            "blue" => Balls::Blue(
                                ball.trim()
                                    .split(" ")
                                    .next()
                                    .unwrap()
                                    .trim()
                                    .parse::<usize>()
                                    .unwrap(),
                            ),
                            "red" => Balls::Red(
                                ball.trim()
                                    .split(" ")
                                    .next()
                                    .unwrap()
                                    .trim()
                                    .parse::<usize>()
                                    .unwrap(),
                            ),
                            "green" => Balls::Green(
                                ball.trim()
                                    .split(" ")
                                    .next()
                                    .unwrap()
                                    .trim()
                                    .parse::<usize>()
                                    .unwrap(),
                            ),
                            x => panic!("Ball type wrong: '{}' for string '{}'", x, ball),
                        }
                    })
                })
                .flatten()
                .map(|b| match b {
                    Balls::Red(n) => n > 12,
                    Balls::Green(n) => n > 13,
                    Balls::Blue(n) => n > 14,
                })
                .any(|x| x)
            {
                None
            } else {
                Some(i + 1)
            }
        })
        .sum()
}
