use rayon::prelude::*;
use regex::Regex;
const EXAMPLE_ANSWER: usize = 48;

fn main() {
    let sample = include_str!("sample2.txt");
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
    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();

    let re_num = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|h| h.as_str())
        .fold((true, vec![]), |(mut d, mut muls), instruction| {
            if instruction == "do()" {
                d = true;
            } else if instruction == "don't()" {
                d = false;
            } else if d {
                muls.push(instruction);
            };
            //println!("{}: {},{:?}", instruction, d, muls);
            (d, muls)
        })
        .1
        .into_iter()
        .map(move |m| {
            re_num
                .find_iter(m)
                .map(|n| n.as_str().parse::<usize>().unwrap())
                .product::<usize>()
        })
        .sum()
}
