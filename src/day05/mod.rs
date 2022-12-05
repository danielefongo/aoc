use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use regex::Regex;

use crate::utils::{lines, read_input};

#[derive(Debug)]
struct Cargo {
    crates: HashMap<usize, VecDeque<Crate>>,
}
impl Cargo {
    fn apply(&mut self, movement: &Move) {
        (0..movement.quantity).into_iter().for_each(|_| {
            let krate = self.pop_from_top(movement.from).unwrap();

            self.put_on_top(movement.to, krate);
        });
    }
    fn top_crates(&self) -> Vec<String> {
        (0..self.crates.len())
            .map(|idx| {
                self.crates
                    .get(&idx)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .clone()
                    .to_string()
            })
            .collect()
    }
    fn put_on_top(&mut self, idx: usize, krate: Crate) {
        (self.crates.entry(idx)).or_default().push_front(krate);
    }
    fn put_on_end(&mut self, idx: usize, krate: Crate) {
        (self.crates.entry(idx)).or_default().push_back(krate);
    }
    fn pop_from_top(&mut self, idx: usize) -> Option<Crate> {
        self.crates.get_mut(&idx).unwrap().pop_front()
    }
}
impl From<Vec<String>> for Cargo {
    fn from(lines: Vec<String>) -> Self {
        let mut cargo = Cargo {
            crates: HashMap::new(),
        };

        lines.iter().for_each(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|it| it.to_vec().iter().collect::<String>())
                .map(|it| it.trim().to_owned())
                .enumerate()
                .filter(|(_, c)| !c.is_empty())
                .for_each(|(idx, c)| {
                    cargo.put_on_end(idx, c.clone().into());
                });
        });

        cargo
    }
}

#[derive(Debug)]
struct Crate(String);
impl From<String> for Crate {
    fn from(string: String) -> Self {
        Self(string.replace("[", "").replace("]", ""))
    }
}
impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}
impl Move {
    fn new(quantity: usize, from: usize, to: usize) -> Self {
        Self { quantity, from, to }
    }
}
impl From<String> for Move {
    fn from(line: String) -> Self {
        let re = Regex::new("move \\d+ from \\d+ to \\d+").unwrap();
        let re2 = Regex::new("\\d+").unwrap();

        if !re.is_match(&line) {
            panic!("invalid input")
        };

        let numbers: Vec<usize> = re2
            .find_iter(&line)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        Self::new(numbers[0], numbers[1] - 1, numbers[2] - 1)
    }
}

pub fn run() {
    let input = lines(read_input(5));

    let (mut config, moves): (Vec<String>, Vec<String>) =
        input.into_iter().partition(|it| !it.starts_with("move"));
    config.remove(config.len() - 1);

    let mut cargo: Cargo = config.into();

    let moves: Vec<Move> = moves.into_iter().map(Into::into).collect();
    moves.iter().for_each(|movement| {
        cargo.apply(movement);
    });

    println!("Part1: {:?}", cargo.top_crates().join(""));
}

#[cfg(test)]
mod tests {
    use super::{Cargo, Move};

    #[test]
    fn get_top() {
        let input = vec!["[A]".to_owned(), "[C] [B]".to_owned()];
        let cargo: Cargo = input.into();

        assert_eq!(cargo.top_crates(), vec!["A", "B"]);
    }

    #[test]
    fn movement() {
        let input = vec!["[A]".to_owned(), "[C] [B]".to_owned()];
        let movement = Move::new(1, 0, 1);

        let mut cargo: Cargo = input.into();
        cargo.apply(&movement);

        assert_eq!(cargo.top_crates(), vec!["C", "A"]);
    }
}
