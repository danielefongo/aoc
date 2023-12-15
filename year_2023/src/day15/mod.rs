use std::collections::HashMap;

use utils::{extract_one, read_input};

#[derive(Debug)]
enum Op {
    Add(u32),
    Remove,
}

#[derive(Debug)]
struct Sequence {
    label: String,
    op: Op,
}
impl Sequence {
    fn find_box(&self) -> u32 {
        ascii_hash(&self.label)
    }
}
impl From<String> for Sequence {
    fn from(value: String) -> Self {
        let label = extract_one(&value, "\\w+");
        let op = if value.contains("=") {
            Op::Add(extract_one(&value, "\\d+").parse().unwrap())
        } else {
            Op::Remove
        };

        Self { label, op }
    }
}

pub fn run() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2());
}

fn part1() -> u32 {
    read_input!()
        .replace('\n', "")
        .split(',')
        .map(ascii_hash)
        .sum::<u32>()
}

type BoxNumber = u32;
type LenseLabel = String;
type LensePower = u32;

fn part2() -> u32 {
    let mut lenses: HashMap<BoxNumber, Vec<(LenseLabel, LensePower)>> = HashMap::new();

    read_input!()
        .replace('\n', "")
        .split(',')
        .map(|it| it.to_string())
        .map(Sequence::from)
        .for_each(|seq| {
            let lenses = lenses.entry(seq.find_box()).or_default();
            let maybe_existing_pos = lenses.iter().position(|(label, _)| label == &seq.label);

            match seq.op {
                Op::Add(power) => {
                    if let Some(existing_pos) = maybe_existing_pos {
                        (*lenses)[existing_pos] = (seq.label.clone(), power);
                    } else {
                        (*lenses).push((seq.label.clone(), power));
                    }
                }
                Op::Remove => {
                    if let Some(already_existing) = maybe_existing_pos {
                        (*lenses).remove(already_existing);
                    }
                }
            };
        });

    lenses
        .iter()
        .flat_map(|(box_number, box_lenses)| {
            box_lenses
                .iter()
                .enumerate()
                .map(|(idx, (_, power))| (box_number + 1) * (idx as u32 + 1) * power)
                .collect::<Vec<_>>()
        })
        .sum::<u32>()
}

fn ascii_hash(word: &str) -> u32 {
    word.chars()
        .fold(0, |acc, act| ((acc + u32::from(act)) * 17) % 256)
}
