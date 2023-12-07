use std::{cmp::Ordering, collections::HashMap};

use utils::{lines, read_input};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(u32),
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            char => Self::Number(char.to_string().parse().unwrap()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rule {
    Default,
    Jokers,
}
impl Rule {
    fn card_strength(&self, card: &Card) -> u32 {
        match self {
            Self::Default => match card {
                Card::A => 14,
                Card::K => 13,
                Card::Q => 12,
                Card::J => 11,
                Card::T => 10,
                Card::Number(number) => *number,
            },
            Rule::Jokers => match card {
                Card::A => 14,
                Card::K => 13,
                Card::Q => 12,
                Card::J => 0,
                Card::T => 10,
                Card::Number(number) => *number,
            },
        }
    }
    fn cards_strength(&self, cards: &[Card]) -> Vec<u32> {
        cards
            .iter()
            .map(|c| self.card_strength(c))
            .collect::<Vec<_>>()
    }
    fn hand_type(&self, hand: &Hand) -> HandType {
        let mut hash: HashMap<Card, u32> = HashMap::new();
        hand.cards.iter().for_each(|card| {
            *hash.entry(card.clone()).or_default() += 1;
        });

        if self == &Rule::Jokers {
            if hand.cards.iter().all(|it| it == &Card::J) {
                return HandType::FiveOfAKind;
            }

            let j = *hash.get(&Card::J).unwrap_or(&0);
            let best = hash
                .iter()
                .filter(|c| c.0 != &Card::J)
                .max_by(|a, b| a.1.cmp(b.1))
                .unwrap()
                .0
                .clone();

            *hash.get_mut(&best).unwrap() += j;
            *hash.get_mut(&Card::J).unwrap_or(&mut 0) = 0;
        }

        [
            |cards| rule(cards, &[(5, 1)], HandType::FiveOfAKind),
            |cards| rule(cards, &[(4, 1)], HandType::FourOfAKind),
            |cards| rule(cards, &[(2, 1), (3, 1)], HandType::FullHouse),
            |cards| rule(cards, &[(3, 1)], HandType::ThreeOfAKind),
            |cards| rule(cards, &[(2, 2)], HandType::TwoPair),
            |cards| rule(cards, &[(2, 1)], HandType::OnePair),
        ]
        .iter()
        .find_map(|f| f(&hash))
        .unwrap_or(HandType::HighCard)
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
    rule: Rule,
    bid: u32,
}
impl Hand {
    fn new(cards: Vec<Card>, rule: Rule, bid: u32) -> Self {
        Self { cards, rule, bid }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_hand_type = self.rule.hand_type(self);
        let other_hand_type = self.rule.hand_type(other);
        let self_cards = self.rule.cards_strength(&self.cards);
        let other_cards = self.rule.cards_strength(&other.cards);

        match self_hand_type.cmp(&other_hand_type) {
            Ordering::Equal => other_cards.cmp(&self_cards),
            ordering => ordering,
        }
    }
}

fn rule(
    cards: &HashMap<Card, u32>,
    rule: &[(u32, u32)],
    return_type: HandType,
) -> Option<HandType> {
    rule.iter()
        .all(|(combination, count)| {
            cards.iter().filter(|(_, c)| c == &combination).count() == *count as usize
        })
        .then_some(return_type)
}

pub fn run() {
    println!("Part1: {:?}", runner(Rule::Default));
    println!("Part2: {:?}", runner(Rule::Jokers));
}

fn runner(rule: Rule) -> u32 {
    let mut hands = lines(read_input!())
        .into_iter()
        .map(|value| {
            let data = value.split(' ').collect::<Vec<_>>();
            let cards = data[0].chars().map(Card::from).collect::<Vec<_>>();
            let bid = data[1].parse().unwrap();
            Hand::new(cards, rule, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| b.cmp(a));

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1) as u32)
        .sum::<u32>()
}
