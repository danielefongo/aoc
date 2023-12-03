use std::{collections::HashMap, fmt::Display, iter::Sum};

use utils::{lines, read_input};

pub fn run() {
    let snafus: Vec<Snafu> = lines(read_input!()).into_iter().map(Snafu::from).collect();
    let sum = snafus.iter().sum::<Snafu>();
    println!("Part1: {}", sum);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Decimal(i64);
impl From<Snafu> for Decimal {
    fn from(snafu: Snafu) -> Self {
        Decimal(
            snafu
                .values
                .iter()
                .rev()
                .enumerate()
                .map(|(pow, value)| value * 5_i64.pow(pow as u32))
                .sum(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Snafu {
    values: Vec<i64>,
}
impl Snafu {
    fn new(values: Vec<i64>) -> Self {
        Self { values }
    }
}
impl<'a> Sum<&'a Snafu> for Snafu {
    fn sum<I: Iterator<Item = &'a Snafu>>(iter: I) -> Self {
        let decimal = Decimal(iter.map(|it| Decimal::from(it.clone()).0).sum());
        Self::from(decimal)
    }
    // fn sum<I: Iterator<Item = &Snafu>>(iter: I) -> Self {
    //     let decimal = Decimal(iter.map(Decimal::from).map(|it| it.0).sum());
    //     Self::from(decimal)
    // }
    // fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    //     let decimal = Decimal(iter.map(Decimal::from).map(|it| it.0).sum());
    //     Self::from(decimal)
    // }
}
impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self
            .values
            .iter()
            .map(|it| match *it {
                -2 => "=".to_string(),
                -1 => "-".to_string(),
                x => x.to_string(),
            })
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", value)
    }
}
impl From<String> for Snafu {
    fn from(input: String) -> Self {
        let values: Vec<i64> = input
            .chars()
            .map(|it| match it {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("invalid input"),
            })
            .collect();
        Self::new(values)
    }
}
impl From<Decimal> for Snafu {
    fn from(decimal: Decimal) -> Self {
        let mut x = decimal.0;
        let mut value: HashMap<u32, i64> = HashMap::new();
        while x != 0 {
            let xa = x.abs();
            let xs = x.signum();

            let power = Counter::new(0)
                .find(|pow| five_pow(pow + 1) / 2 >= xa)
                .unwrap();
            let candidate = five_pow(power);

            let digit = if candidate as f32 * 1.5 < xa as f32 {
                2
            } else {
                1
            };

            value.insert(power, digit * xs);
            x -= candidate * digit * xs;
        }

        let values = (0..=*value.keys().max().unwrap())
            .map(|it| *value.get(&it).unwrap_or(&0))
            .rev()
            .collect::<Vec<i64>>();

        Self::new(values)
    }
}

struct Counter {
    actual: u32,
}
impl Counter {
    fn new(actual: u32) -> Self {
        Self { actual }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.actual += 1;
        Some(self.actual - 1)
    }
}

fn five_pow(power: u32) -> i64 {
    5_i64.pow(power)
}

#[cfg(test)]
mod tests {
    use crate::day25::{Decimal, Snafu};

    fn snafu(input: &str) -> Snafu {
        Snafu::from(input.to_string())
    }

    #[test]
    fn convert_string_to_snafu() {
        assert_eq!(Snafu::from("1".to_string()), Snafu::new(vec![1]));
        assert_eq!(Snafu::from("2".to_string()), Snafu::new(vec![2]));
        assert_eq!(Snafu::from("1=".to_string()), Snafu::new(vec![1, -2]));
        assert_eq!(
            Snafu::from("1-0---0".to_string()),
            Snafu::new(vec![1, -1, 0, -1, -1, -1, 0])
        );
    }

    #[test]
    fn convert_to_decimal() {
        assert_eq!(Decimal::from(snafu("1")), Decimal(1));
        assert_eq!(Decimal::from(snafu("2")), Decimal(2));
        assert_eq!(Decimal::from(snafu("1=")), Decimal(3));
        assert_eq!(Decimal::from(snafu("1-0---0")), Decimal(12345));
    }

    #[test]
    fn convert_to_snafu() {
        assert_eq!(Snafu::from(Decimal(1)), snafu("1"));
        assert_eq!(Snafu::from(Decimal(2022)), snafu("1=11-2"));
        assert_eq!(Snafu::from(Decimal(12345)), snafu("1-0---0"));
        assert_eq!(Snafu::from(Decimal(314159265)), snafu("1121-1110-1=0"));
    }
}
