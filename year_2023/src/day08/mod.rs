use std::collections::HashMap;

use utils::{extract, lines, read_input};

type StringMapping = HashMap<String, (String, String)>;

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

#[derive(Clone, Debug)]
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
    map: StringMapping,
}
impl Mappings {
    fn next(&self, movement: Movement, actual: &String) -> String {
        match movement {
            Movement::Left => self.map.get(actual).unwrap().0.clone(),
            Movement::Right => self.map.get(actual).unwrap().1.clone(),
        }
    }

    fn get_steps_by_ending(
        &mut self,
        mut movements: Movements,
        starting: String,
        end_condition: fn(&String) -> bool,
    ) -> Vec<(u64, String)> {
        let mut endings = Vec::new();
        let mut steps_by_ending = Vec::new();
        let mut count = 0;
        let mut actual = starting.clone();

        loop {
            actual = self.next(movements.next().unwrap(), &actual);
            count += 1;
            if endings.contains(&(movements.actual, actual.clone())) {
                break;
            }
            if end_condition(&actual) {
                endings.push((movements.actual, actual.clone()));
                steps_by_ending.push((count, actual.clone()))
            }
        }

        steps_by_ending
    }
}
impl From<Vec<String>> for Mappings {
    fn from(value: Vec<String>) -> Self {
        let mut map: StringMapping = HashMap::new();

        value.into_iter().for_each(|line| {
            let key = line.split(" = ").collect::<Vec<_>>()[0];
            let value = line.split(" = ").collect::<Vec<_>>()[1];
            let elements = extract(value, "\\w+");

            map.insert(key.to_string(), (elements[0].clone(), elements[1].clone()));
        });

        Self { map }
    }
}

pub fn run() {
    println!(
        "Part1: {}",
        runner(
            |map| vec![map.keys().find(|it| *it == "AAA").unwrap().to_string()],
            |actual| actual == "ZZZ"
        )
    );
    println!(
        "Part2: {}",
        runner(
            |map| map
                .keys()
                .filter(|it| it.ends_with('A'))
                .cloned()
                .collect::<Vec<_>>(),
            |actual| actual.ends_with('Z')
        )
    );
}

fn runner(elements: fn(&StringMapping) -> Vec<String>, end_condition: fn(&String) -> bool) -> u64 {
    let lines = lines(read_input!());
    let movements = Movements::from(lines[0].clone());
    let mut mappings = Mappings::from(lines.into_iter().skip(1).collect::<Vec<_>>());

    let acts = elements(&mappings.map);

    let mut total = vec![];
    for x in acts {
        total.append(&mut mappings.get_steps_by_ending(movements.clone(), x, end_condition))
    }

    vector_lcm(&total.into_iter().map(|(v, _)| v).collect::<Vec<_>>())
}

fn vector_lcm(numbers: &[u64]) -> u64 {
    let mut result = 1;

    for &num in numbers {
        result = lcm(result, num);
    }
    result
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
