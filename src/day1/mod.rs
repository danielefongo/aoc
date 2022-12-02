use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    part1(read_input("./src/day1/input"));
}

fn part1(input: String) {
    let max_resources: u32 = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&inner_input| {
            inner_input
                .split("\n")
                .filter(|it| !it.trim().is_empty())
                .map(|it| it.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap();

    println!("{:?}", max_resources);
}

fn read_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data
}
