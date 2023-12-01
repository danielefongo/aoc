use std::{cmp::Ordering, iter::Peekable};

use utils::{lines, read_input};

macro_rules! single {
    ($data:expr) => {{
        List::Single($data)
    }};
}
macro_rules! multiple {
    () => {
        List::Multiple(vec![])
    };
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec : Vec<crate::day13::List> = Vec::new();
            $(
                temp_vec.push($x);
            )*
            List::Multiple(temp_vec)
        }
    };
}

fn parse_list(input: &str) -> List {
    let mut input = input.chars().into_iter().peekable();
    if let Some(c) = input.peek() {
        match c {
            '0'..='9' => return build_single(&mut input),
            '[' => {
                input.next();
                return build_multiple(&mut input);
            }
            _ => panic!("invalid input"),
        }
    }
    panic!("no input");
}

fn build_multiple(input: &mut Peekable<impl Iterator<Item = char>>) -> List {
    let mut result: Vec<List> = vec![];
    while let Some(c) = input.peek() {
        match c {
            '0'..='9' => result.push(build_single(input)),
            '[' => {
                input.next();
                result.push(build_multiple(input))
            }
            ']' => {
                input.next();
                return List::Multiple(result);
            }
            ' ' | ',' => {
                input.next();
            }
            a => panic!("invalid input {}", a),
        }
    }
    panic!("no input");
}

fn build_single(input: &mut Peekable<impl Iterator<Item = char>>) -> List {
    let mut number = String::new();
    while let Some(c) = input.peek() {
        if matches!(c, '0'..='9') {
            number.push_str(&input.next().unwrap().to_string());
        } else {
            break;
        }
    }

    List::Single(number.parse().unwrap())
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum List {
    Single(usize),
    Multiple(Vec<List>),
}
impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List::Multiple(l), List::Multiple(r)) => {
                for idx in 0..l.len() {
                    if let Some(right) = r.get(idx) {
                        match l.get(idx).unwrap().cmp(&right) {
                            Ordering::Equal => continue,
                            ord => return ord,
                        }
                    }
                }
                l.len().cmp(&r.len())
            }
            (List::Multiple(_), List::Single(_)) => self.cmp(&other.to_list()),
            (List::Single(_), List::Multiple(_)) => self.to_list().cmp(&other),
            (List::Single(l), List::Single(r)) => l.cmp(r),
        }
    }
}
impl List {
    fn at(&self, idx: usize) -> Option<&List> {
        match self {
            List::Single(_) => None,
            List::Multiple(v) => v.get(idx),
        }
    }
    fn to_list(&self) -> List {
        match self {
            List::Multiple(_) => self.clone(),
            List::Single(_) => multiple!(self.clone()),
        }
    }
}

fn compare(input1: &str, input2: &str) -> Ordering {
    let left = parse_list(input1);
    let right = parse_list(input2);
    left.cmp(&right)
}

pub fn run() {
    let mut lines = lines(read_input!());
    println!("Part1: {}", part1(&lines));

    lines.push("[[2]]".to_owned());
    lines.push("[[6]]".to_owned());
    println!("Part2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    lines
        .chunks(2)
        .enumerate()
        .filter(|(_, lines)| {
            let left = parse_list(&lines[0]);
            let right = parse_list(&lines[1]);
            !matches!(left.cmp(&right), Ordering::Greater)
        })
        .map(|(idx, _)| idx + 1)
        .sum::<usize>()
}

fn part2(lines: &Vec<String>) -> usize {
    let mut sortable_list = lines.iter().map(|it| parse_list(it)).collect::<Vec<List>>();

    sortable_list.sort();

    sortable_list
        .into_iter()
        .enumerate()
        .filter(|(_, it)| {
            it == &multiple!(multiple!(single!(2))) || it == &multiple!(multiple!(single!(6)))
        })
        .map(|(idx, _)| idx + 1)
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    mod parsing {
        use crate::day13::{parse_list, List};

        #[test]
        fn single_element() {
            let expected = single!(12);
            assert_eq!(parse_list("12"), expected);
        }

        #[test]
        fn single_element_list() {
            let expected = multiple!(single!(12));
            assert_eq!(parse_list("[12]"), expected);
        }

        #[test]
        fn multiple_list() {
            let expected = multiple!(single!(12), single!(21));
            assert_eq!(parse_list("[12, 21]"), expected);
        }

        #[test]
        fn nested_list() {
            let expected = multiple!(multiple!(single!(12)));
            assert_eq!(parse_list("[[12]]"), expected);
        }

        #[test]
        fn mixed_list() {
            let expected = multiple!(multiple!(single!(12)), single!(1), multiple!());
            assert_eq!(parse_list("[[12], 1, []]"), expected);
        }
    }

    mod solving {
        use std::cmp::Ordering;

        use crate::day13::compare;

        #[test]
        fn single() {
            assert_eq!(compare("1", "1"), Ordering::Equal);
            assert_eq!(compare("1", "2"), Ordering::Less);
            assert_eq!(compare("2", "1"), Ordering::Greater);
        }

        #[test]
        fn left_empty() {
            assert_eq!(compare("[]", "[1]"), Ordering::Less);
            assert_eq!(compare("[]", "[]"), Ordering::Equal);
        }

        #[test]
        fn same_len() {
            assert_eq!(compare("[1, 2]", "[1, 2]"), Ordering::Equal);
            assert_eq!(compare("[1, 2]", "[1, 3]"), Ordering::Less);
            assert_eq!(compare("[1, 3]", "[1, 1]"), Ordering::Greater);
        }

        #[test]
        fn left_bigger() {
            assert_eq!(compare("[1, 2]", "[1]"), Ordering::Greater);
            assert_eq!(compare("[1, 2]", "[2]"), Ordering::Less);
        }

        #[test]
        fn mixed() {
            assert_eq!(compare("[[1, 2]]", "[2]"), Ordering::Less);
            assert_eq!(compare("[[1, 2]]", "[[[3]]]"), Ordering::Less);

            assert_eq!(compare("[[1, 2]]", "[1]"), Ordering::Greater);
            assert_eq!(compare("[[1, 2]]", "[[[1]]]"), Ordering::Greater);
        }
    }
}
