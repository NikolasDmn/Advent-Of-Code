use rayon::prelude::*;
const EXAMPLE_ANSWER: usize = 281;

fn main() {
    let sample = include_str!("sample2.txt");
    let input = include_str!("input.txt");
    let sample_answer = solve(sample);
    if sample_answer != EXAMPLE_ANSWER {
        eprintln!(
            "Algorithm wrong from example:\nExpected{}\nProvided{}",
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
        .lines()
        .collect::<Vec<&str>>()
        .par_iter()
        .map(|&line| {
            line.chars()
                .enumerate()
                .map(|(i, c)| {
                    for (num_s, num) in [
                        ("one", '1'),
                        ("two", '2'),
                        ("three", '3'),
                        ("four", '4'),
                        ("five", '5'),
                        ("six", '6'),
                        ("seven", '7'),
                        ("eight", '8'),
                        ("nine", '9'),
                    ] {
                        if line[i..].starts_with(num_s) {
                            return num;
                        }
                    }
                    return c;
                })
                .collect::<String>()
        })
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first) as usize;
            first as usize * 10 + last
        })
        .sum()
}
