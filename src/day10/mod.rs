use crate::utils::{lines, read_input};

#[derive(Debug)]
enum Op {
    Add(i32),
    Noop,
}
impl From<String> for Op {
    fn from(input: String) -> Self {
        if input == "noop" {
            Op::Noop
        } else if input.starts_with("addx") {
            let (_, value) = input.split_once(" ").unwrap();
            Op::Add(value.parse().unwrap())
        } else {
            panic!("invalid input")
        }
    }
}

#[derive(Debug)]
struct Register {
    history: Vec<i32>,
}
impl Register {
    fn new() -> Self {
        Self { history: vec![1] }
    }
    fn run(&mut self, op: Op) {
        let last_value = self.history.last().unwrap_or(&1).clone();

        match op {
            Op::Add(value) => {
                self.history.push(last_value);
                self.history.push(last_value + value)
            }
            Op::Noop => {
                self.history.push(last_value);
            }
        }
    }
    fn value_during(&self, step_number: usize) -> i32 {
        self.history.get(step_number - 1).unwrap().clone()
    }
}

pub fn run() {
    let mut register = Register::new();

    lines(read_input(10))
        .into_iter()
        .map(Op::from)
        .for_each(|a| register.run(a));

    let values = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|it| register.value_during(it as usize) * it)
        .sum::<i32>();

    println!("Part1: {}", values);
}
