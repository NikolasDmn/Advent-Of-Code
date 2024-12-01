use rayon::prelude::*;
const EXAMPLE_ANSWER: usize = 2286;

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
        .map(|(i, line)| {
            line.split(":")
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
                .fold(
                    (0, 0, 0, 0),
                    |(mul, mut max_red, mut max_green, mut max_blue), ball| {
                        match ball {
                            Balls::Red(n) => {
                                if n > max_red {
                                    max_red = n
                                }
                            }
                            Balls::Green(n) => {
                                if n > max_green {
                                    max_green = n
                                }
                            }
                            Balls::Blue(n) => {
                                if n > max_blue {
                                    max_blue = n
                                }
                            }
                        }
                        (max_red * max_blue * max_green, max_red, max_green, max_blue)
                    },
                )
                .0
        })
        .sum()
}
