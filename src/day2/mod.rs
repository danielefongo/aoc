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
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissor,
            _ => panic!("Invalid input"),
        }
    }
}

pub fn run() {
    part1(read_input(2));
}

fn part1(input: String) {
    let data: Vec<(Hand, Hand)> = input
        .split("\n")
        .filter(|it| !it.is_empty())
        .map(parse_line)
        .collect();

    let points: u32 = data
        .iter()
        .map(|(other_hand, hand)| hand.points(other_hand))
        .sum();

    println!("{:?}", points);
}

fn parse_line(line: &str) -> (Hand, Hand) {
    let hands = line
        .split(" ")
        .map(Into::into)
        .take(2)
        .collect::<Vec<Hand>>();

    (hands[0].clone(), hands[1].clone())
}
