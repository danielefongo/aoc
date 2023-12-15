use utils::read_input;

pub fn run() {
    println!(
        "Part1: {}",
        read_input!()
            .replace('\n', "")
            .split(',')
            .map(|word| {
                word.chars()
                    .fold(0, |acc, act| ((acc + u32::from(act)) * 17) % 256)
            })
            .sum::<u32>()
    )
}
