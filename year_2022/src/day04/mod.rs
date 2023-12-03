use utils::{lines, read_input};

#[derive(Clone)]
struct Range {
    min: u32,
    max: u32,
}
impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }
    fn intersect_left(&self, other: &Range) -> bool {
        self.max >= other.min && self.min <= other.min
    }
}
impl From<String> for Range {
    fn from(input: String) -> Self {
        let values: Vec<u32> = input.split('-').map(|it| it.parse().unwrap()).collect();

        Self {
            min: values[0],
            max: values[1],
        }
    }
}

pub fn run() {
    println!("Part1: {}", count(filter_contains));
    println!("Part2: {}", count(filter_intersect));
}

fn count(filter: impl FnMut(&(Range, Range)) -> bool) -> usize {
    lines(read_input!())
        .into_iter()
        .map(to_ranges)
        .filter(filter)
        .count()
}

fn filter_contains((r1, r2): &(Range, Range)) -> bool {
    r1.contains(r2) || r2.contains(r1)
}

fn filter_intersect((r1, r2): &(Range, Range)) -> bool {
    r1.intersect_left(r2) || r2.intersect_left(r1)
}

fn to_ranges(input: String) -> (Range, Range) {
    let inputs = input
        .split(',')
        .map(|it| it.to_owned())
        .map(Into::into)
        .collect::<Vec<Range>>();

    (inputs[0].clone(), inputs[1].clone())
}
