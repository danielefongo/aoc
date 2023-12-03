use std::convert::identity;

use utils::{lines, read_input};

pub fn run() {
    println!("Part1: {}", run_part(lines(read_input!()), identity));
    println!("Part2: {}", run_part(lines(read_input!()), replace_digits));
}

fn run_part(input: Vec<String>, mapper: impl Fn(String) -> String) -> u32 {
    input
        .into_iter()
        .map(mapper)
        .map(|line| {
            let first = line.chars().filter(char::is_ascii_digit).take(1);
            let second = line.chars().filter(char::is_ascii_digit).rev().take(1);
            first.chain(second).collect::<String>()
        })
        .map(|numbers_line| numbers_line.parse::<u32>().unwrap_or_default())
        .sum()
}

fn replace_digits(line: String) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}
