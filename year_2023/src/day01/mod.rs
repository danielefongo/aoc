use utils::{lines, read_input};

pub fn run() {
    println!("Part1: {}", part1(lines(read_input!())));
}

fn part1(input: Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|line| {
            let first = line.chars().filter(char::is_ascii_digit).take(1);
            let second = line.chars().filter(char::is_ascii_digit).rev().take(1);
            first.chain(second).collect::<String>()
        })
        .map(|numbers_line| numbers_line.parse::<u32>().unwrap_or_default())
        .sum()
}
