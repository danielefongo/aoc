use std::{iter::Zip, vec::IntoIter};

use utils::{lines, read_input};

fn transpose(strings: &[String]) -> Vec<String> {
    (0..strings[0].len())
        .map(|i| {
            strings
                .iter()
                .map(|s| s.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect()
}

#[derive(Debug)]
enum MirrorPos {
    None,
    Vertical(usize),
    Horizontal(usize),
}
impl MirrorPos {
    fn score(self) -> usize {
        match self {
            MirrorPos::Vertical(value) => value,
            MirrorPos::Horizontal(value) => value * 100,
            MirrorPos::None => 0,
        }
    }
}

enum Rule {
    NoSmudges,
    Smudges,
}
impl Rule {
    fn compare(&self, first: u32, second: u32) -> bool {
        match self {
            Self::NoSmudges => first == second,
            Self::Smudges => (first ^ second).count_ones() <= 1,
        }
    }
    fn valid_mirror(&self, zip: Zip<IntoIter<&u32>, IntoIter<&u32>>) -> bool {
        match self {
            Rule::NoSmudges => zip.clone().all(|(a, b)| a == b),
            Rule::Smudges => zip.map(|(a, b)| (a ^ b).count_ones()).sum::<u32>() == 1,
        }
    }
}

#[derive(Debug)]
struct Mirrors {
    vertical: Vec<u32>,
    horizontal: Vec<u32>,
}
impl Mirrors {
    fn find_mirror(&self, rule: Rule) -> MirrorPos {
        if let Some(pos) = self.find_mirror_pos(&self.vertical, &rule) {
            return MirrorPos::Vertical(pos + 1);
        }
        if let Some(pos) = self.find_mirror_pos(&self.horizontal, &rule) {
            return MirrorPos::Horizontal(pos + 1);
        }

        MirrorPos::None
    }
    fn find_mirror_pos(&self, lines: &Vec<u32>, rule: &Rule) -> Option<usize> {
        (0..(lines.len() - 1))
            .filter(|it| rule.compare(lines[*it], lines[it + 1]))
            .find(|pos| {
                let left = lines.iter().take(pos + 1).rev().collect::<Vec<_>>();
                let right = lines.iter().skip(pos + 1).collect::<Vec<_>>();
                rule.valid_mirror(left.into_iter().zip(right))
            })
    }
}
impl From<Vec<String>> for Mirrors {
    fn from(value: Vec<String>) -> Self {
        Self {
            vertical: transpose(&value)
                .iter()
                .map(|it| it.replace('#', "1").replace('.', "0"))
                .map(|it| u32::from_str_radix(&it, 2).unwrap())
                .collect(),
            horizontal: value
                .iter()
                .map(|it| it.replace('#', "1").replace('.', "0"))
                .map(|it| u32::from_str_radix(&it, 2).unwrap())
                .collect(),
        }
    }
}

pub fn run() {
    println!(
        "Part1: {:?}",
        read_input!()
            .split("\n\n")
            .map(|block| Mirrors::from(lines(block.to_string())))
            .map(|mirrors| mirrors.find_mirror(Rule::NoSmudges))
            .map(MirrorPos::score)
            .sum::<usize>()
    );
    println!(
        "Part2: {:?}",
        read_input!()
            .split("\n\n")
            .map(|block| Mirrors::from(lines(block.to_string())))
            .map(|mirrors| mirrors.find_mirror(Rule::Smudges))
            .map(MirrorPos::score)
            .sum::<usize>()
    );
}
