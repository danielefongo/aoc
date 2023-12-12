use std::collections::HashMap;

use utils::{extract, lines, read_input};

#[derive(Clone, Copy, Debug)]
enum Movement {
    Left,
    Right,
}
impl From<char> for Movement {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug)]
struct Movements {
    movements: Vec<Movement>,
    actual: usize,
}
impl From<String> for Movements {
    fn from(value: String) -> Self {
        Self {
            movements: value.chars().map(Movement::from).collect(),
            actual: 0,
        }
    }
}
impl Iterator for Movements {
    type Item = Movement;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.movements[self.actual];
        self.actual = (self.actual + 1) % self.movements.len();
        Some(next)
    }
}

#[derive(Debug)]
struct Mappings {
    map: HashMap<String, (String, String)>,
    actual: String,
}
impl Mappings {
    fn next(&mut self, movement: Movement) {
        match movement {
            Movement::Left => self.actual = self.map.get(&self.actual).unwrap().0.clone(),
            Movement::Right => self.actual = self.map.get(&self.actual).unwrap().1.clone(),
        }
    }
}
impl From<Vec<String>> for Mappings {
    fn from(value: Vec<String>) -> Self {
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        let mut actual: Option<String> = None;
        value.into_iter().for_each(|line| {
            let key = line.split(" = ").collect::<Vec<_>>()[0];
            let value = line.split(" = ").collect::<Vec<_>>()[1];
            let elements = extract(value, "\\w+");

            if key == "AAA" {
                actual = Some(key.to_string())
            }
            map.insert(key.to_string(), (elements[0].clone(), elements[1].clone()));
        });
        Self {
            map,
            actual: actual.unwrap(),
        }
    }
}

pub fn run() {
    let lines = lines(read_input!());
    let mut movements = Movements::from(lines[0].clone());
    let mut mappings = Mappings::from(lines.into_iter().skip(1).collect::<Vec<_>>());

    let mut count = 0;
    while mappings.actual != *"ZZZ" {
        mappings.next(movements.next().unwrap());
        count += 1;
    }
    println!("{:?}", count);
}
