use utils::{extract, lines, read_input};

fn replace(vec: &[char], skip: usize, take: usize, char: char) -> Vec<char> {
    let mut new_vec = vec.to_vec();
    new_vec
        .iter_mut()
        .skip(skip)
        .take(take)
        .for_each(|it| *it = char);
    new_vec
}

#[derive(Debug)]
struct Case {
    value: Vec<char>,
    elements: Vec<usize>,
    actual: usize,
}
impl Case {
    fn is_valid(&self) -> bool {
        !self.value.iter().skip(self.actual).any(|it| it == &'#') && self.elements.is_empty()
    }
    fn generate(&self) -> Vec<Case> {
        if self.elements.is_empty() || self.actual >= self.value.len() {
            return vec![];
        };

        match self.value.get(self.actual).unwrap() {
            '?' => {
                return vec![
                    Case {
                        value: replace(&self.value, self.actual, 1, '.'),
                        actual: self.actual + 1,
                        elements: self.elements.clone(),
                    },
                    Case {
                        value: replace(&self.value, self.actual, 1, '#'),
                        actual: self.actual,
                        elements: self.elements.clone(),
                    },
                ];
            }
            '#' => {
                if let Some(first) = self.elements.first() {
                    let number_of_valid_chars = self
                        .value
                        .iter()
                        .skip(self.actual)
                        .take(*first)
                        .filter(|it| *it == &'?' || *it == &'#')
                        .count();
                    if number_of_valid_chars >= *first
                        && self.value.get(self.actual + number_of_valid_chars) != Some(&'#')
                    {
                        let filled_hashes = replace(&self.value, self.actual, *first, '#');
                        let filled_dot = replace(&filled_hashes, self.actual + *first, 1, '.');

                        return vec![Case {
                            value: filled_dot,
                            actual: self.actual + *first + 1,
                            elements: self.elements.iter().skip(1).cloned().collect(),
                        }];
                    }
                }
            }
            '.' => {
                return vec![Case {
                    value: self.value.clone(),
                    actual: self.actual + 1,
                    elements: self.elements.clone(),
                }];
            }
            _ => panic!("Invalid char"),
        }

        vec![]
    }
}
impl From<String> for Case {
    fn from(line: String) -> Self {
        Self {
            value: line.split(' ').collect::<Vec<_>>()[0].chars().collect(),
            elements: extract(line.split(' ').collect::<Vec<_>>()[1], "\\d+")
                .into_iter()
                .map(|it| it.parse::<usize>().unwrap())
                .collect(),
            actual: 0,
        }
    }
}

pub fn run() {
    let mut cases = lines(read_input!())
        .into_iter()
        .map(Case::from)
        .collect::<Vec<_>>();

    let mut count = 0;
    while let Some(case) = cases.pop() {
        if case.is_valid() {
            count += 1;
            continue;
        }
        let mut others = case.generate();
        cases.append(&mut others);
    }
    println!("Part1: {}", count);
}
