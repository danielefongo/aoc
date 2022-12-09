use std::collections::HashSet;

use crate::utils::{lines, read_input};

#[derive(Debug, Clone)]
enum Step {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Steps {
    steps: Vec<Step>,
}
impl From<Vec<String>> for Steps {
    fn from(lines: Vec<String>) -> Self {
        let steps = lines
            .into_iter()
            .map(|it| {
                let data: Vec<&str> = it.split(" ").collect();
                let step = match data[0] {
                    "L" => Step::Left,
                    "R" => Step::Right,
                    "U" => Step::Up,
                    "D" => Step::Down,
                    _ => panic!("invalid input"),
                };
                let count = data[1].parse::<i32>().expect("invalid input");

                (0..count).map(|_| step.clone()).collect::<Vec<Step>>()
            })
            .flatten()
            .collect();
        Self { steps }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    path: HashSet<(i32, i32)>,
}
impl Point {
    fn new() -> Self {
        let mut point = Self {
            x: 0,
            y: 0,
            path: HashSet::new(),
        };
        point.update_path();
        point
    }
    fn walk(&mut self, step: &Step) {
        match step {
            Step::Left => self.x -= 1,
            Step::Right => self.x += 1,
            Step::Up => self.y += 1,
            Step::Down => self.y -= 1,
        }
        self.update_path();
    }
    fn follow(&mut self, other: &Point) {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
            return;
        }

        if x_diff.abs() >= 1 {
            self.x += x_diff / x_diff.abs();
        }

        if y_diff.abs() >= 1 {
            self.y += y_diff / y_diff.abs();
        }

        self.update_path();
    }
    fn update_path(&mut self) {
        self.path.insert((self.x, self.y));
    }
}

pub fn run() {
    let steps: Steps = lines(read_input(9)).into();
    let mut head = Point::new();
    let mut tail = Point::new();
    steps.steps.into_iter().for_each(|s| {
        head.walk(&s);
        tail.follow(&head);
    });

    println!("Part1: {:?}", tail.path.len());
}
