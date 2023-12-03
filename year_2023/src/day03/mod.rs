use utils::{lines, read_input};

#[derive(Debug, Clone, Copy)]
struct Pos(usize, usize);
impl Pos {
    fn is_next_to(&self, other: Pos) -> bool {
        (self.0).abs_diff(other.0) <= 1 && (self.1).abs_diff(other.1) <= 1
    }
}

#[derive(Debug)]
struct SchemeNumber(u32, Vec<Pos>);
impl SchemeNumber {
    fn is_next_to_symbol(&self, symbol: &SchemeSymbol) -> bool {
        self.1.iter().any(|pos| pos.is_next_to(symbol.1))
    }
    fn number(&self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
struct SchemeSymbol(char, Pos);

pub fn run() {
    let mut numbers: Vec<SchemeNumber> = vec![];
    let mut symbols: Vec<SchemeSymbol> = vec![];

    let mut actual_number = String::new();
    for (row, line) in lines(read_input!()).iter_mut().enumerate() {
        line.push('.');
        for (col, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                actual_number.push_str(&char.to_string());
            } else if !actual_number.is_empty() {
                numbers.push(SchemeNumber(
                    actual_number.parse().unwrap(),
                    ((col - actual_number.len())..col)
                        .map(|col| Pos(row, col))
                        .collect(),
                ));
                actual_number = String::new();
            }

            if !char.is_ascii_digit() && char != '.' {
                symbols.push(SchemeSymbol(char, Pos(row, col)))
            }
        }

        actual_number = String::new();
    }

    println!(
        "Part1: {:?}",
        numbers
            .iter()
            .filter(|number| {
                symbols
                    .iter()
                    .any(|symbol| number.is_next_to_symbol(symbol))
            })
            .map(|number| number.number())
            .sum::<u32>()
    );

    println!(
        "Part2: {:?}",
        symbols
            .iter()
            .map(|symbol| {
                let next_numbers = numbers
                    .iter()
                    .filter(|number| number.is_next_to_symbol(symbol))
                    .collect::<Vec<_>>();
                if next_numbers.len() > 1 {
                    next_numbers.iter().map(|it| it.number()).product::<u32>()
                } else {
                    0
                }
            })
            .sum::<u32>()
    );
}
