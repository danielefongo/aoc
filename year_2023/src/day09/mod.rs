use utils::{extract, lines, read_input};

#[derive(Clone, Debug)]
struct Sequence {
    numbers: Vec<i32>,
}
impl From<String> for Sequence {
    fn from(value: String) -> Self {
        Self {
            numbers: extract(&value, "-?\\d+")
                .into_iter()
                .map(|it| it.parse::<i32>().unwrap())
                .collect(),
        }
    }
}
impl Sequence {
    fn diff(&self) -> Self {
        Self {
            numbers: (0..(self.numbers.len() - 1))
                .map(|idx| self.numbers[idx + 1] - self.numbers[idx])
                .collect(),
        }
    }
    fn is_completed(&self) -> bool {
        self.numbers.iter().all(|it| it == &0)
    }
    fn predict(&self) -> i32 {
        let mut next = self.clone();
        let mut values = vec![];
        loop {
            next = next.diff();
            if next.is_completed() {
                break;
            }
            values.push(*next.numbers.last().unwrap());
        }
        self.numbers.last().unwrap() + values.into_iter().sum::<i32>()
    }
}

pub fn run() {
    println!(
        "Part1: {:?}",
        lines(read_input!())
            .into_iter()
            .map(Sequence::from)
            .map(|it| it.predict())
            .sum::<i32>()
    );
}
