use std::{cmp::Ordering, collections::HashMap};

use utils::{lines, read_input};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card(u32);
impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card(14),
            'K' => Card(13),
            'Q' => Card(12),
            'J' => Card(11),
            'T' => Card(10),
            char => Card(char.to_string().parse().unwrap()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => other.cards.cmp(&self.cards),
            ordering => ordering,
        }
    }
}
impl From<String> for Hand {
    fn from(value: String) -> Self {
        let data = value.split(' ').collect::<Vec<_>>();
        let cards = data[0].chars().map(Card::from).collect::<Vec<_>>();
        let bid = data[1].parse().unwrap();

        let hash = counters(&cards);

        let hand_type = [
            five_of_a_kind,
            four_of_a_kind,
            full_house,
            three_of_kind,
            two_pair,
            one_pair,
        ]
        .iter()
        .find_map(|f| f(&hash))
        .unwrap_or(HandType::HighCard);

        Hand {
            cards,
            hand_type,
            bid,
        }
    }
}

fn counters(cards: &[Card]) -> HashMap<u32, u32> {
    let mut hash: HashMap<u32, u32> = HashMap::new();
    cards.iter().for_each(|card| {
        *hash.entry(card.0).or_default() += 1;
    });
    hash
}

fn five_of_a_kind(cards: &HashMap<u32, u32>) -> Option<HandType> {
    cards
        .iter()
        .any(|(_, c)| c == &5)
        .then_some(HandType::FiveOfAKind)
}

fn four_of_a_kind(cards: &HashMap<u32, u32>) -> Option<HandType> {
    cards
        .iter()
        .any(|(_, c)| c == &4)
        .then_some(HandType::FourOfAKind)
}

fn full_house(cards: &HashMap<u32, u32>) -> Option<HandType> {
    if cards.iter().any(|(_, c)| c == &2) && cards.iter().any(|(_, c)| c == &3) {
        Some(HandType::FullHouse)
    } else {
        None
    }
}

fn three_of_kind(cards: &HashMap<u32, u32>) -> Option<HandType> {
    cards
        .iter()
        .any(|(_, c)| c == &3)
        .then_some(HandType::ThreeOfAKind)
}

fn two_pair(cards: &HashMap<u32, u32>) -> Option<HandType> {
    (cards.iter().filter(|(_, c)| **c == 2).count() == 2).then_some(HandType::TwoPair)
}

fn one_pair(cards: &HashMap<u32, u32>) -> Option<HandType> {
    (cards.iter().filter(|(_, c)| **c == 2).count() == 1).then_some(HandType::OnePair)
}

pub fn run() {
    let mut hands = lines(read_input!())
        .into_iter()
        .map(Hand::from)
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| b.cmp(a));

    println!(
        "{:?}",
        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| { hand.bid * (idx + 1) as u32 })
            .sum::<u32>()
    );
}
