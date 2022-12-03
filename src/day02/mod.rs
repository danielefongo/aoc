use crate::utils::read_input;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}
impl Hand {
    fn points(&self, other: &Hand) -> u32 {
        let points = match (self, other) {
            (Self::Rock, Self::Scissor) => 6,
            (Self::Scissor, Self::Paper) => 6,
            (Self::Paper, Self::Rock) => 6,
            (x, y) if x == y => 3,
            _ => 0,
        };

        points + self._extra_points()
    }
    fn _extra_points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }
}
impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        match input {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissor,
            _ => panic!("Invalid input"),
        }
    }
}
impl From<Strategy> for Hand {
    fn from(input: Strategy) -> Self {
        match input {
            Strategy::One(value) => match &value[..] {
                "X" => Self::Rock,
                "Y" => Self::Paper,
                "Z" => Self::Scissor,
                _ => panic!("Invalid input"),
            },
            Strategy::Two(strategy, other_hand) => match (&strategy[..], other_hand) {
                ("X", Hand::Rock) => Hand::Scissor,
                ("X", Hand::Paper) => Hand::Rock,
                ("X", Hand::Scissor) => Hand::Paper,
                ("Y", hand) => hand,
                ("Z", Hand::Rock) => Hand::Paper,
                ("Z", Hand::Paper) => Hand::Scissor,
                ("Z", Hand::Scissor) => Hand::Rock,
                _ => panic!("Invalid input"),
            },
        }
    }
}

enum Strategy {
    One(String),
    Two(String, Hand),
}

pub fn run() {
    let input = read_input(2);
    let strategy = strategy_2;

    let data: Vec<(Hand, Hand)> = input
        .split("\n")
        .filter(|it| !it.is_empty())
        .map(strategy)
        .collect();

    let points: u32 = data
        .iter()
        .map(|(other_hand, hand)| hand.points(other_hand))
        .sum();

    println!("{:?}", points);
}

fn strategy_1(line: &str) -> (Hand, Hand) {
    let hands = line.split(" ").take(2).collect::<Vec<&str>>();
    let other_hand = hands[0].into();
    let my_hand: Hand = Strategy::One(hands[1].to_owned()).into();

    (other_hand, my_hand)
}

fn strategy_2(line: &str) -> (Hand, Hand) {
    let hands = line.split(" ").take(2).collect::<Vec<&str>>();
    let other_hand: Hand = hands[0].into();
    let my_hand: Hand = Strategy::Two(hands[1].to_owned(), other_hand.clone()).into();

    (other_hand, my_hand)
}
