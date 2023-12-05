use std::collections::HashSet;

use utils::{extract, extract_one, read_input};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Range {
    from: i64,
    to: i64,
    offset: i64,
}
impl Range {
    fn intersect(&self, r2: &Range) -> Option<Range> {
        if r2.from > self.to || self.from > r2.to {
            None
        } else {
            Some(Range {
                from: self.from.max(r2.from) + r2.offset,
                to: self.to.min(r2.to) + r2.offset,
                offset: 0,
            })
        }
    }

    fn intersect_many(&self, ranges: &[Range]) -> HashSet<Range> {
        let min = ranges.iter().map(|it| it.from).min().unwrap_or(self.from);
        let max = ranges.iter().map(|it| it.to).max().unwrap_or(self.to);

        let mut found_ranges = ranges
            .iter()
            .filter_map(|it| self.intersect(it))
            .collect::<HashSet<_>>();

        if min <= self.to && min >= self.from {
            found_ranges.insert(Range {
                from: self.from,
                to: min - 1,
                offset: 0,
            });
        }
        if max >= self.from && max <= self.to {
            found_ranges.insert(Range {
                from: max + 1,
                to: self.to,
                offset: 0,
            });
        }
        if max < self.from || min > self.to {
            found_ranges.insert(Range {
                from: self.from,
                to: self.to,
                offset: 0,
            });
        }
        found_ranges
    }
}
impl From<String> for Range {
    fn from(value: String) -> Self {
        let numbers = extract(&value, "\\d+");
        let destination: i64 = numbers[0].parse().unwrap();
        let from: i64 = numbers[1].parse().unwrap();
        let length: i64 = numbers[2].parse().unwrap();
        Self {
            from,
            to: from + length - 1,
            offset: destination - from,
        }
    }
}

#[derive(Debug)]
struct Mapper {
    ranges: Vec<Range>,
}
impl Mapper {
    fn find_ranges(&self, sources: Vec<Range>) -> Vec<Range> {
        sources
            .iter()
            .flat_map(|range| range.intersect_many(&self.ranges))
            .collect()
    }
}
impl From<String> for Mapper {
    fn from(value: String) -> Self {
        let category_string = extract_one(&value, ".* map:");
        let ranges = value
            .replace(&category_string, "")
            .split('\n')
            .filter(|it| !it.is_empty())
            .map(|it| Range::from(it.to_string()))
            .collect();
        Mapper { ranges }
    }
}

pub fn run() {
    println!("Part1: {}", runner(gen_range_1));
    println!("Part2: {}", runner(gen_range_2))
}

fn runner(gen_range: fn(Vec<i64>) -> Vec<Range>) -> i64 {
    let lines = read_input!()
        .split("\n\n")
        .map(|it| it.to_owned())
        .collect::<Vec<_>>();

    let seed_ranges = gen_range(
        extract(&lines[0], "\\d+")
            .into_iter()
            .map(|it| it.parse().unwrap())
            .collect::<Vec<_>>(),
    );

    let mappers = lines
        .into_iter()
        .skip(1)
        .map(Mapper::from)
        .collect::<Vec<_>>();

    mappers
        .iter()
        .fold(seed_ranges, |sources, mapper| mapper.find_ranges(sources))
        .into_iter()
        .map(|it| it.from)
        .min()
        .unwrap()
}

fn gen_range_1(seeds: Vec<i64>) -> Vec<Range> {
    seeds
        .iter()
        .map(|value| Range {
            from: *value,
            to: *value,
            offset: 0,
        })
        .collect()
}

fn gen_range_2(seeds: Vec<i64>) -> Vec<Range> {
    seeds
        .chunks(2)
        .map(|values| {
            let from = values[0];
            let lenght = values[1];
            Range {
                from,
                to: from + lenght - 1,
                offset: 0,
            }
        })
        .collect()
}
