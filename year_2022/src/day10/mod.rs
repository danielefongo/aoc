use std::ops::Deref;

use utils::{lines, read_input};

#[derive(Debug)]
enum MemoryOp {
    Add(i32),
    Noop,
}
impl From<String> for MemoryOp {
    fn from(input: String) -> Self {
        if input == "noop" {
            MemoryOp::Noop
        } else if input.starts_with("addx") {
            let (_, value) = input.split_once(" ").unwrap();
            MemoryOp::Add(value.parse().unwrap())
        } else {
            panic!("invalid input")
        }
    }
}

#[derive(Debug, Clone)]
enum CpuOp {
    Add(i32),
    Loading,
    Noop,
}

#[derive(Debug, Clone)]
struct CpuOps {
    ops: Vec<CpuOp>,
}
impl From<Vec<MemoryOp>> for CpuOps {
    fn from(ops: Vec<MemoryOp>) -> Self {
        let ops: Vec<CpuOp> = ops
            .iter()
            .map(|it| match it {
                MemoryOp::Add(val) => vec![CpuOp::Loading, CpuOp::Add(val.clone())],
                MemoryOp::Noop => vec![CpuOp::Noop],
            })
            .flatten()
            .collect();
        Self { ops }
    }
}
impl Deref for CpuOps {
    type Target = Vec<CpuOp>;

    fn deref(&self) -> &Self::Target {
        &self.ops
    }
}

#[derive(Debug)]
struct Register {
    actual: i32,
    step: usize,
    ops: CpuOps,
}
impl Register {
    fn new(ops: CpuOps) -> Self {
        Self {
            actual: 1,
            step: 0,
            ops,
        }
    }
    fn execute(&mut self, op: CpuOp) {
        if let CpuOp::Add(val) = op {
            self.actual += val;
        }
        self.step += 1;
    }
}
impl Iterator for Register {
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == 0 {
            self.execute(CpuOp::Noop);
        } else {
            let op = self.ops.get(self.step - 1)?;
            self.execute(op.clone());
        }
        Some((self.step, self.actual))
    }
}

struct CRT {
    register: Register,
    lines: Vec<String>,
}
impl CRT {
    fn new(ops: CpuOps) -> Self {
        Self {
            register: Register::new(ops),
            lines: Vec::new(),
        }
    }
    fn get_line(&mut self, line: usize) -> Option<&mut String> {
        while self.lines.get(line).is_none() {
            self.lines
                .push("........................................".to_owned());
        }

        self.lines.get_mut(line)
    }
    fn print_on_line(&mut self, line: usize, pixel: usize) {
        self.get_line(line)
            .unwrap()
            .replace_range(pixel..(pixel + 1), "#");
    }
    fn run(&mut self) {
        while let Some((step, register_value)) = self.register.next() {
            let step = step as i32;
            let line_idx = (step - 1) / 40;
            let pixel_idx = (step - 1) % 40;
            let (pixel_min, pixel_max) = (register_value - 1, register_value + 1);

            if pixel_idx >= pixel_min && pixel_idx <= pixel_max {
                self.print_on_line(line_idx as usize, pixel_idx as usize)
            }
        }
    }
}

pub fn run() {
    let ops: CpuOps = lines(read_input!())
        .into_iter()
        .map(MemoryOp::from)
        .collect::<Vec<MemoryOp>>()
        .into();

    part1(ops.clone());
    part2(ops);
}

fn part1(ops: CpuOps) {
    let register = Register::new(ops);

    println!(
        "Part1: {:?}",
        register
            .skip(19)
            .step_by(40)
            .map(|(idx, value)| value * (idx as i32))
            .sum::<i32>()
    );
}

fn part2(ops: CpuOps) {
    let mut crt = CRT::new(ops);
    crt.run();
    println!("Part2:");
    crt.lines.iter().for_each(|it| println!("{}", it));
}
