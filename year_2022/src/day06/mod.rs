use std::{collections::HashSet, hash::Hash};

use utils::read_input;

pub fn run() {
    let input = read_input!();

    println!("Part1: {}", find_marker(&input, 4));
    println!("Part2: {}", find_marker(&input, 14));
}

fn find_marker(input: &str, distinct_chars: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(distinct_chars)
        .enumerate()
        .find(|(_, window)| all_unique(window))
        .map(|(i, _)| i + distinct_chars)
        .unwrap()
}

fn all_unique<T: Ord + Clone + Hash>(data: &[T]) -> bool {
    data.iter().cloned().collect::<HashSet<T>>().len() == data.len()
}
