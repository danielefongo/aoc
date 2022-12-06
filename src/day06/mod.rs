use crate::utils::read_input;

pub fn run() {
    let input = read_input(6);

    let marker = (0..input.len())
        .find(|&i| all_unique(input[i..(i + 4)].chars().collect::<Vec<char>>()))
        .map(|i| i + 4)
        .unwrap();

    println!("Part1: {}", marker);
}

fn all_unique<T: Ord + Clone>(data: Vec<T>) -> bool {
    let mut data2 = data.clone();
    data2.sort();
    data2.dedup();

    data2.len() == data.len()
}
