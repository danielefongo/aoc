use std::collections::{HashMap, VecDeque};

use utils::{extract, extract_one, read_input};

#[derive(Debug)]
struct Throw {
    to: usize,
    item: Item,
}

#[derive(Debug)]
struct Item {
    value: usize,
}
impl From<&String> for Item {
    fn from(input: &String) -> Self {
        Self {
            value: input.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Value {
    Old,
    Int(usize),
}
impl Value {
    fn get(&self, value: &usize) -> usize {
        match self {
            Value::Old => value.clone(),
            Value::Int(val) => val.clone(),
        }
    }
}
impl From<String> for Value {
    fn from(input: String) -> Self {
        if let Ok(val) = input.parse() {
            Value::Int(val)
        } else {
            Value::Old
        }
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl Op {
    fn execute(&self, left: usize, right: usize) -> usize {
        match self {
            Op::Add => left + right,
            Op::Subtract => left - right,
            Op::Multiply => left * right,
            Op::Divide => left / right,
        }
    }
}
impl From<String> for Op {
    fn from(input: String) -> Self {
        match &input[..] {
            "+" => Op::Add,
            "-" => Op::Subtract,
            "*" => Op::Multiply,
            "/" => Op::Divide,
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug)]
struct Operation {
    left: Value,
    op: Op,
    right: Value,
}
impl Operation {
    fn execute(&self, input: usize) -> usize {
        let left = self.left.get(&input);
        let right = self.right.get(&input);
        self.op.execute(left, right)
    }
}
impl From<String> for Operation {
    fn from(input: String) -> Self {
        let data = input
            .split(" ")
            .map(|it| it.to_owned())
            .collect::<Vec<String>>();

        Self {
            left: data[0].clone().into(),
            op: data[1].clone().into(),
            right: data[2].clone().into(),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    inspections: usize,
    items: VecDeque<Item>,
    operation: Operation,
    to_if_false: usize,
    to_if_true: usize,
    condition: usize,
    bound: usize,
    worry_level_divider: usize,
}
impl Monkey {
    fn do_round(&mut self) -> Vec<Throw> {
        let mut throws: Vec<Throw> = vec![];

        while let Some(mut item) = self.items.pop_front() {
            item.value =
                (self.operation.execute(item.value) / self.worry_level_divider) % self.bound;
            let to = if item.value % self.condition == 0 {
                self.to_if_true
            } else {
                self.to_if_false
            };
            self.inspections += 1;
            throws.push(Throw { to, item })
        }
        throws
    }
    fn accept_item(&mut self, item: Item) {
        self.items.push_back(item);
    }
}
impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let inputs = input.split("\n").collect::<Vec<&str>>();
        let items = extract(inputs[1], "\\d+")
            .iter()
            .map(|it| it.into())
            .collect::<VecDeque<Item>>();
        let operation: Operation =
            extract_one(inputs[2], "(\\w+|\\d+) (\\+|-|\\*|/) (\\w+|\\d+)").into();
        let condition: usize = extract_one(inputs[3], "\\d+").parse().unwrap();
        let to_if_true: usize = extract_one(inputs[4], "\\d+").parse().unwrap();
        let to_if_false: usize = extract_one(inputs[5], "\\d+").parse().unwrap();
        Monkey {
            items,
            operation,
            to_if_false,
            to_if_true,
            condition,
            worry_level_divider: 3,
            bound: usize::MAX,
            inspections: 0,
        }
    }
}

pub fn run() {
    part1(&mut generate_monkeys());
    part2(&mut generate_monkeys());
}

fn part1(monkeys: &mut HashMap<usize, Monkey>) {
    println!("Part1: {:?}", run_rounds(monkeys, 20));
}

fn part2(monkeys: &mut HashMap<usize, Monkey>) {
    let bound = monkeys
        .values()
        .map(|it| it.condition)
        .reduce(|a, b| a * b)
        .unwrap();
    (0..monkeys.len()).for_each(|monkey_idx| {
        let mut monkey = monkeys.get_mut(&monkey_idx).unwrap();
        monkey.bound = bound;
        monkey.worry_level_divider = 1;
    });

    println!("Part2: {:?}", run_rounds(monkeys, 10000));
}

fn run_rounds(monkeys: &mut HashMap<usize, Monkey>, rounds: usize) -> usize {
    (0..rounds).for_each(|_round| {
        (0..monkeys.len()).for_each(|monkey_idx| {
            let monkey = monkeys.get_mut(&monkey_idx).unwrap();
            let throws = monkey.do_round();
            throws.into_iter().for_each(|it| {
                monkeys.get_mut(&it.to).unwrap().accept_item(it.item);
            });
        });
    });

    let mut inspections = monkeys
        .values()
        .map(|it| it.inspections)
        .collect::<Vec<usize>>();
    inspections.sort_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1]
}

fn generate_monkeys() -> HashMap<usize, Monkey> {
    let input = read_input!();
    let input: Vec<&str> = input.split("\n\n").collect();
    input.into_iter().map(Monkey::from).enumerate().collect()
}
