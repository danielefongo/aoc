use std::collections::HashMap;

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
    fn combinations(&self) -> usize {
        let mut hashmap = HashMap::new();
        self.combinations_for_indexes(&mut hashmap, 0, 0)
    }
    fn combinations_for_indexes(
        &self,
        cache: &mut HashMap<(usize, usize), usize>,
        char_idx: usize,
        element_idx: usize,
    ) -> usize {
        if cache.contains_key(&(char_idx, element_idx)) {
            return *cache.get(&(char_idx, element_idx)).unwrap();
        }

        let value = if let Some(element) = self.elements.get(element_idx) {
            match self.value.get(char_idx) {
                Some('.') => self.combinations_for_indexes(cache, char_idx + 1, element_idx),
                Some('?') => {
                    let z = self.combinations_for_indexes(cache, char_idx + 1, element_idx);
                    if self.can_fit(char_idx, *element) {
                        z + self.combinations_for_indexes(
                            cache,
                            char_idx + element + 1,
                            element_idx + 1,
                        )
                    } else {
                        z
                    }
                }
                Some('#') => {
                    if self.can_fit(char_idx, *element) {
                        self.combinations_for_indexes(
                            cache,
                            char_idx + element + 1,
                            element_idx + 1,
                        )
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        } else {
            usize::from(self.value.iter().skip(char_idx).all(|it| it != &'#'))
        };

        cache.insert((char_idx, element_idx), value);
        value
    }
    fn unfolded(&self) -> Self {
        Self {
            value: (0..5)
                .map(|_| self.value.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("?")
                .chars()
                .collect(),
            elements: (0..5).flat_map(|_| self.elements.clone()).collect(),
            actual: self.actual,
        }
    }
    fn can_fit(&self, char_idx: usize, element: usize) -> bool {
        let any_dot = self
            .value
            .iter()
            .skip(char_idx)
            .take(element)
            .any(|it| it == &'.');
        let hash_after = self.value.get(char_idx + element) == Some(&'#');
        let overflow = char_idx + element > self.value.len();

        !any_dot && !hash_after && !overflow
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
    println!(
        "Part1: {}",
        lines(read_input!())
            .into_iter()
            .map(Case::from)
            .map(|it| it.combinations())
            .sum::<usize>()
    );
    println!(
        "Part2: {}",
        lines(read_input!())
            .into_iter()
            .map(Case::from)
            .map(|it| it.unfolded().combinations())
            .sum::<usize>()
    );
}
