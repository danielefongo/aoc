use std::{collections::HashMap, fmt::Display};

use utils::{lines, read_input};

#[derive(Debug, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Space,
    CubeShapedRock,
    RoundedRock,
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Space => write!(f, "."),
            Element::CubeShapedRock => write!(f, "#"),
            Element::RoundedRock => write!(f, "O"),
        }
    }
}
impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Space,
            '#' => Self::CubeShapedRock,
            'O' => Self::RoundedRock,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Debug)]
struct Platform {
    elements: HashMap<Pos, Element>,
    width: i32,
    height: i32,
}
impl Platform {
    fn tilt(&mut self) {
        (0..self.width).for_each(|x| {
            let mut target_y = 0;
            (0..self.height).for_each(|y| {
                let element = self.elements.get(&Pos::new(x, y)).unwrap().clone();
                match element {
                    Element::Space => {}
                    Element::CubeShapedRock => {
                        target_y = y + 1;
                    }
                    Element::RoundedRock => {
                        self.elements.insert(Pos::new(x, y), Element::Space);
                        self.elements.insert(Pos::new(x, target_y), element);
                        target_y += 1;
                    }
                }
            });
        })
    }
    fn score(&self) -> i32 {
        let mut count = 0;
        for pos in self
            .elements
            .iter()
            .filter_map(|(pos, el)| (el == &Element::RoundedRock).then_some(pos))
        {
            count += self
                .elements
                .iter()
                .filter(|(other_pos, _)| other_pos.y >= pos.y && other_pos.x == pos.x)
                .count() as i32;
        }
        count
    }
}
impl From<Vec<String>> for Platform {
    fn from(lines: Vec<String>) -> Self {
        Self {
            width: lines[0].len() as i32,
            height: lines.len() as i32,
            elements: lines
                .into_iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, char)| (Pos::new(x as i32, y as i32), Element::from(char)))
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
}

pub fn run() {
    let mut platform = Platform::from(lines(read_input!()));
    platform.tilt();
    println!("Part1: {}", platform.score());
}
