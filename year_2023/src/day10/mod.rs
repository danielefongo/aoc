use std::collections::HashMap;

use utils::{lines, read_input};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn sum(&self, delta: Delta) -> Self {
        Self {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
type Delta = Pos;

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, PartialEq, Eq)]
enum PipeType {
    Vertical,
    Horizontal,
    L,
    J,
    Seven,
    F,
    Ground,
    Animal,
}
impl From<char> for PipeType {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Animal,
            '.' => Self::Ground,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::L,
            'J' => Self::J,
            '7' => Self::Seven,
            'F' => Self::F,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Debug)]
struct Pipe {
    pipe_type: PipeType,
    pos: Pos,
}
impl From<(i32, i32, char)> for Pipe {
    fn from(value: (i32, i32, char)) -> Self {
        Self {
            pipe_type: PipeType::from(value.2),
            pos: Pos {
                x: value.0,
                y: value.1,
            },
        }
    }
}
impl Pipe {
    fn next(&self, dir: &Direction) -> Option<(Delta, Direction)> {
        match (&self.pipe_type, dir) {
            (&PipeType::Vertical, &Direction::Down) => Some((Delta::new(0, 1), Direction::Down)),
            (&PipeType::Vertical, &Direction::Up) => Some((Delta::new(0, -1), Direction::Up)),
            (&PipeType::Horizontal, &Direction::Right) => {
                Some((Delta::new(1, 0), Direction::Right))
            }
            (&PipeType::Horizontal, &Direction::Left) => Some((Delta::new(-1, 0), Direction::Left)),
            (&PipeType::L, &Direction::Down) => Some((Delta::new(1, 0), Direction::Right)),
            (&PipeType::L, &Direction::Left) => Some((Delta::new(0, -1), Direction::Up)),
            (&PipeType::J, &Direction::Right) => Some((Delta::new(0, -1), Direction::Up)),
            (&PipeType::J, &Direction::Down) => Some((Delta::new(-1, 0), Direction::Left)),
            (&PipeType::Seven, &Direction::Right) => Some((Delta::new(0, 1), Direction::Down)),
            (&PipeType::Seven, &Direction::Up) => Some((Delta::new(-1, 0), Direction::Left)),
            (&PipeType::F, &Direction::Left) => Some((Delta::new(0, 1), Direction::Down)),
            (&PipeType::F, &Direction::Up) => Some((Delta::new(1, 0), Direction::Right)),
            (PipeType::Animal, Direction::Right) => Some((Delta::new(1, 0), Direction::Right)),
            (PipeType::Animal, Direction::Down) => Some((Delta::new(0, 1), Direction::Down)),
            (PipeType::Animal, Direction::Left) => Some((Delta::new(-1, 0), Direction::Left)),
            (PipeType::Animal, Direction::Up) => Some((Delta::new(0, -1), Direction::Up)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Map {
    pipes: HashMap<Pos, Pipe>,
}
impl Map {
    fn get_max_distance(&self, mut dir: Direction) -> Option<i32> {
        let mut next = self
            .pipes
            .values()
            .find(|it| it.pipe_type == PipeType::Animal)?;

        let mut count = 0;

        while let Some((delta, new_dir)) = next.next(&dir) {
            dir = new_dir;
            next = self.pipes.get(&next.pos.sum(delta))?;
            count += 1;
            if next.pipe_type == PipeType::Animal {
                return Some(count / 2);
            }
        }

        None
    }
}
impl From<Vec<String>> for Map {
    fn from(lines: Vec<String>) -> Self {
        Self {
            pipes: lines
                .into_iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, char)| {
                            let x: i32 = x as i32;
                            let y: i32 = y as i32;
                            (Pos::new(x, y), Pipe::from((x, y, char)))
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
}

pub fn run() {
    let map = Map::from(lines(read_input!()));
    println!(
        "Part1: {}",
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left
        ]
        .into_iter()
        .filter_map(|dir| map.get_max_distance(dir))
        .max()
        .unwrap()
    )
}
