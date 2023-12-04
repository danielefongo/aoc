use utils::{extract, extract_one, lines, read_input};

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}
impl Card {
    fn points(&self) -> u32 {
        let matching_numbers = self.won_cards();
        if matching_numbers > 0 {
            2_i32.pow(matching_numbers - 1) as u32
        } else {
            0
        }
    }
    fn won_cards(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }
}
impl From<String> for Card {
    fn from(value: String) -> Self {
        let id = extract_one(&value, "\\d+").parse().unwrap();
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
            id,
            winning_numbers,
            numbers,
        }
    }
}

pub fn run() {
    let cards = lines(read_input!())
        .into_iter()
        .map(Card::from)
        .collect::<Vec<_>>();

    println!("Part1: {}", part1(&cards));
    println!("Part2: {}", part2(&cards));
}

fn part1(cards: &[Card]) -> u32 {
    cards.iter().map(|it| it.points()).sum::<u32>()
}
fn part2(cards: &[Card]) -> u32 {
    let mut count = 0;
    let mut stack = cards.iter().map(Clone::clone).collect::<Vec<_>>();
    while let Some(actual_card) = stack.pop() {
        count += 1;
        cards
            .iter()
            .skip(actual_card.id)
            .take(actual_card.won_cards() as usize)
            .for_each(|c| stack.push(c.clone()));
    }
    count
}
