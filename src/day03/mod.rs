use crate::utils::{lines, read_input};

pub fn run() {
    println!("Part1: {}", part1(read_input(3)));
    println!("Part2: {}", part2(read_input(3)));
}

fn part1(input: String) -> u32 {
    lines(input)
        .into_iter()
        .map(split_bag)
        .map(find_common_item)
        .sum::<u32>()
}

fn part2(input: String) -> u32 {
    lines(input)
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .map(find_common_item)
        .sum::<u32>()
}

fn split_bag(line: String) -> Vec<String> {
    let compartment_size = line.len() / 2;
    let first_compartment = line[0..compartment_size].to_owned();
    let second_compartment = line[compartment_size..line.len()].to_owned();

    vec![first_compartment, second_compartment]
}

fn find_common_item(data: Vec<String>) -> u32 {
    let first = data.first().unwrap();
    let others: Vec<String> = data.iter().cloned().skip(1).collect();

    let character = first
        .chars()
        .find(|it| {
            others
                .iter()
                .all(|other| other.find(it.to_owned()).is_some())
        })
        .unwrap();

    let value = match character {
        'A'..='Z' => (character as u8) - 65 + 27,
        'a'..='z' => (character as u8) - 97 + 1,
        _ => panic!("Invalid input"),
    };

    value.into()
}
