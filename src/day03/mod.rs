use crate::utils::read_input;

pub fn run() {
    part1(read_input(3));
}

fn part1(input: String) {
    let result = input
        .split("\n")
        .filter(|it| !it.is_empty())
        .map(find_common_item)
        .sum::<u32>();

    println!("{}", result);
}

fn find_common_item(line: &str) -> u32 {
    let compartment_size = line.len() / 2;
    let first_compartment: &str = &line[0..compartment_size];
    let second_compartment: &str = &line[compartment_size..line.len()];

    let character = first_compartment
        .chars()
        .find(|it| second_compartment.find(it.to_owned()).is_some())
        .unwrap();

    let value = match character {
        'A'..='Z' => (character as u8) - 65 + 27,
        'a'..='z' => (character as u8) - 97 + 1,
        _ => panic!("Invalid input"),
    };

    value.into()
}
