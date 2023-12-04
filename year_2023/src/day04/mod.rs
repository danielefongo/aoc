use utils::{extract, lines, read_input};

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}
impl Card {
    fn points(&self) -> u32 {
        let matching_numbers = self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32;
        if matching_numbers > 0 {
            2_i32.pow(matching_numbers - 1) as u32
        } else {
            0
        }
    }
}
impl From<String> for Card {
    fn from(value: String) -> Self {
        let value = value.split(':').collect::<Vec<_>>()[1];
        let inputs = value.split('|').collect::<Vec<_>>();
        let winning_numbers = extract(inputs[0], "\\d+")
            .into_iter()
            .map(|it| it.parse::<u32>().unwrap())
            .collect();
        let numbers = extract(inputs[1], "\\d+")
            .into_iter()
            .map(|it| it.parse::<u32>().unwrap())
            .collect();

        Self {
            winning_numbers,
            numbers,
        }
    }
}

pub fn run() {
    println!(
        "Part1: {}",
        lines(read_input!())
            .into_iter()
            .map(Card::from)
            .map(|it| it.points())
            .sum::<u32>()
    );
}
