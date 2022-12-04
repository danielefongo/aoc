use crate::utils::{lines, read_input};

#[derive(Clone)]
struct Range {
    min: u32,
    max: u32,
}
impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }
}
impl From<String> for Range {
    fn from(input: String) -> Self {
        let values: Vec<u32> = input.split("-").map(|it| it.parse().unwrap()).collect();

        Self {
            min: values[0],
            max: values[1],
        }
    }
}

pub fn run() {
    let count = lines(read_input(4))
        .iter()
        .map(|it| to_ranges(it))
        .filter(|(r1, r2)| r1.contains(&r2) || r2.contains(&r1))
        .count();

    println!("Part1: {}", count);
}

fn to_ranges(input: &String) -> (Range, Range) {
    let inputs = input
        .split(",")
        .map(|it| it.to_owned())
        .map(Into::into)
        .collect::<Vec<Range>>();

    (inputs[0].clone(), inputs[1].clone())
}
