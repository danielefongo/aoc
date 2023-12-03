use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use utils::{extract_one, lines, matches, read_input};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl Op {
    fn execute(&self, first: &Value, second: &Value) -> Option<i128> {
        match (first, second) {
            (Value::Int(a), Value::Int(b)) => Some(self.inner_execute(a, b)),
            _ => None,
        }
    }
    fn inner_execute(&self, first: &i128, second: &i128) -> i128 {
        match self {
            Op::Add => first + second,
            Op::Subtract => first - second,
            Op::Multiply => first * second,
            Op::Divide => first / second,
        }
    }
}
impl From<&str> for Op {
    fn from(input: &str) -> Self {
        match input {
            "+" => Op::Add,
            "-" => Op::Subtract,
            "*" => Op::Multiply,
            "/" => Op::Divide,
            a => panic!("invalid input {}", a),
        }
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Op::Add => "+".to_string(),
            Op::Subtract => "-".to_string(),
            Op::Multiply => "*".to_string(),
            Op::Divide => "/".to_string(),
        };
        write!(f, "{}", out)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Int(i128),
    Operation(String, Op, String),
    Equality(String, String),
    Unknown,
}
impl From<String> for Value {
    fn from(input: String) -> Self {
        if input.trim().is_empty() {
            Value::Unknown
        } else if matches(&input, "\\d+") {
            Value::Int(input.parse().unwrap())
        } else if matches(&input, ".*=.*") {
            let mut iter = input.split(' ');
            let first = iter.next().unwrap().to_string();
            iter.next().unwrap().to_string();
            let second = iter.next().unwrap().to_string();
            Value::Equality(first, second)
        } else {
            let mut iter = input.split(' ');
            Value::Operation(
                iter.next().unwrap().to_string(),
                iter.next().unwrap().into(),
                iter.next().unwrap().to_string(),
            )
        }
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Value::Int(val) => val.to_string(),
            Value::Operation(first, op, second) => format!(
                "{} {} {}",
                first,
                op,
                second
            ),
            Value::Equality(first, second) => {
                format!("{} = {}", first, second)
            }
            Value::Unknown => "?".to_string(),
        };
        write!(f, "{}", value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Monkey {
    name: String,
    value: Value,
}
impl Monkey {
    fn evaluated(&self) -> bool {
        matches!(self.value, Value::Int(_))
    }
    fn to_eq(&self) -> Self {
        match self.value.clone() {
            Value::Operation(first, _, second) => Monkey::eq(self.name.clone(), first, second),
            _ => panic!("cannot convert to eq"),
        }
    }
    fn unknown(name: String) -> Self {
        Self {
            name,
            value: Value::Unknown,
        }
    }
    fn eq(name: String, first: String, second: String) -> Self {
        Self {
            name,
            value: Value::Equality(first, second),
        }
    }
    fn op(name: String, first: String, op: Op, second: String) -> Self {
        Monkey {
            name,
            value: Value::Operation(first, op, second),
        }
    }
    fn invert_first(&self) -> Option<Monkey> {
        match self.value.clone() {
            Value::Operation(first, op, second) => match op {
                Op::Add => Some(Monkey::op(first, self.name.clone(), Op::Subtract, second)),
                Op::Subtract => Some(Monkey::op(first, self.name.clone(), Op::Add, second)),
                Op::Multiply => Some(Monkey::op(first, self.name.clone(), Op::Divide, second)),
                Op::Divide => Some(Monkey::op(first, self.name.clone(), Op::Multiply, second)),
            },
            _ => None,
        }
    }
    fn invert_second(&self) -> Option<Monkey> {
        match self.value.clone() {
            Value::Operation(first, op, second) => match op {
                Op::Add => Some(Monkey::op(second, self.name.clone(), Op::Subtract, first)),
                Op::Subtract => Some(Monkey::op(second, first, Op::Subtract, self.name.clone())),
                Op::Multiply => Some(Monkey::op(second, self.name.clone(), Op::Divide, first)),
                Op::Divide => Some(Monkey::op(second, first, Op::Divide, self.name.clone())),
            },
            _ => None,
        }
    }
}
impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
impl From<String> for Monkey {
    fn from(input: String) -> Self {
        let name = extract_one(&input, "\\w+");
        let value = extract_one(&input, ": .*").replace(": ", "").into();
        Monkey { name, value }
    }
}

struct Monkeys {
    monkeys: HashMap<String, Monkey>,
}
impl Monkeys {
    fn new(monkeys: HashMap<String, Monkey>) -> Self {
        Self { monkeys }
    }
    fn push(&self, queue: &mut VecDeque<Monkey>, monkey: Monkey) {
        if !queue.contains(&monkey) {
            queue.push_back(monkey);
        }
    }
    fn calculate(&self, target: &str) -> Monkey {
        let mut monkeys = self.monkeys.clone();
        let mut equations: VecDeque<Monkey> = VecDeque::new();
        equations.push_back(monkeys.get("root").unwrap().clone());

        while let Some(monkey) = equations.pop_front() {
            if monkeys.get(&monkey.name).unwrap().evaluated() {
                continue;
            }
            match monkey.value.clone() {
                Value::Int(_) => {}
                Value::Operation(first, op, second) => {
                    let first_monkey = monkeys.get(&first).unwrap();
                    let second_monkey = monkeys.get(&second).unwrap();

                    if let Some(result) = op.execute(&first_monkey.value, &second_monkey.value) {
                        *monkeys.get_mut(&monkey.name).unwrap() = Monkey {
                            name: monkey.name.clone(),
                            value: Value::Int(result),
                        };
                    } else {
                        if !first_monkey.evaluated() {
                            if let Some(m) = first_monkey
                                .invert_first() { self.push(&mut equations, m) }
                            if let Some(m) = first_monkey
                                .invert_second() { self.push(&mut equations, m) }

                            if let Some(m) = monkey.invert_first() { self.push(&mut equations, m) }
                            self.push(&mut equations, first_monkey.clone());
                        }
                        if !second_monkey.evaluated() {
                            if let Some(m) = second_monkey
                                .invert_first() { self.push(&mut equations, m) }
                            if let Some(m) = second_monkey
                                .invert_second() { self.push(&mut equations, m) }

                            if let Some(m) = monkey.invert_second() { self.push(&mut equations, m) }
                            self.push(&mut equations, second_monkey.clone());
                        }

                        self.push(&mut equations, monkey);
                    }
                }
                Value::Equality(first, second) => {
                    let first_monkey = monkeys.get(&first).unwrap();
                    let second_monkey = monkeys.get(&second).unwrap();
                    if first_monkey.evaluated() {
                        *monkeys.get_mut(&second).unwrap() = Monkey {
                            name: second.clone(),
                            value: first_monkey.value.clone(),
                        };
                    } else if second_monkey.evaluated() {
                        *monkeys.get_mut(&first).unwrap() = Monkey {
                            name: first.clone(),
                            value: second_monkey.value.clone(),
                        };
                    } else {
                        self.push(&mut equations, first_monkey.clone());
                        self.push(&mut equations, second_monkey.clone());
                        self.push(&mut equations, monkey);
                    }
                }
                Value::Unknown => {}
            }
        }
        monkeys.get(target).unwrap().clone()
    }
}

pub fn run() {
    let monkeys = lines(read_input!())
        .into_iter()
        .map(|it| it.into())
        .map(|it: Monkey| (it.name.clone(), it))
        .collect::<HashMap<String, Monkey>>();

    let modified_monkeys = monkeys
        .iter()
        .map(|(name, monkey)| {
            let new_monkey = match &name[..] {
                "root" => monkey.to_eq().clone(),
                "humn" => Monkey::unknown(name.to_string()),
                _ => monkey.clone(),
            };
            (name.clone(), new_monkey)
        })
        .collect::<HashMap<_, _>>();

    println!("Part1: {}", Monkeys::new(monkeys).calculate("root"));
    println!(
        "Part2: {}",
        Monkeys::new(modified_monkeys).calculate("humn")
    );
}
