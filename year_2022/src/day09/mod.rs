use std::{cell::RefCell, collections::HashSet, ops::Deref, rc::Rc};

use utils::{lines, read_input};

type RcPoint = Rc<RefCell<Point>>;
type Pos = (i32, i32);

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
impl Deref for Steps {
    type Target = Vec<Step>;

    fn deref(&self) -> &Self::Target {
        &self.steps
    }
}
impl From<Vec<String>> for Steps {
    fn from(lines: Vec<String>) -> Self {
        let steps = lines
            .into_iter()
            .flat_map(|it| {
                let data: Vec<&str> = it.split(' ').collect();
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
            .collect();
        Self { steps }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    tail: Option<RcPoint>,
    path: HashSet<Pos>,
}
impl Point {
    fn new(tail: Option<RcPoint>) -> Self {
        let mut point = Self {
            x: 0,
            y: 0,
            tail,
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
        self.drag_tail();
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
        self.drag_tail();
    }
    fn drag_tail(&mut self) {
        if let Some(tail) = &self.tail {
            tail.borrow_mut().follow(self);
        }
    }
    fn update_path(&mut self) {
        self.path.insert((self.x, self.y));
    }
}

pub fn run() {
    let steps: Steps = lines(read_input!()).into();
    println!("Part1: {:?}", inner_run(1, &steps));
    println!("Part2: {:?}", inner_run(9, &steps));
}

fn inner_run(tails: usize, steps: &Steps) -> usize {
    let tail = Rc::new(RefCell::new(Point::new(None)));
    let head = (0..tails).fold(Rc::clone(&tail), |a, _| {
        Rc::new(RefCell::new(Point::new(Some(a))))
    });

    steps.iter().for_each(|s| {
        head.borrow_mut().walk(s);
    });

    let borrowed_tail = tail.borrow_mut();
    borrowed_tail.path.len()
}
