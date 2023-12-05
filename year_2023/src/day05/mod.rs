use utils::{extract, extract_one, read_input};

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    source_length: u64,
}
impl Range {
    fn find_destination(&self, number: u64) -> Option<u64> {
        if number >= self.source_start && number < self.source_start + self.source_length {
            Some(self.destination_start + number - self.source_start)
        } else {
            None
        }
    }
}
impl From<String> for Range {
    fn from(value: String) -> Self {
        let numbers = extract(&value, "\\d+");
        Self {
            destination_start: numbers[0].parse().unwrap(),
            source_start: numbers[1].parse().unwrap(),
            source_length: numbers[2].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Mapper {
    ranges: Vec<Range>,
}
impl Mapper {
    fn find_destinations(&self, sources: Vec<u64>) -> Vec<u64> {
        sources
            .into_iter()
            .map(|source| {
                self.ranges
                    .iter()
                    .flat_map(|range| range.find_destination(source))
                    .next()
                    .unwrap_or(source)
            })
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
    let lines = read_input!()
        .split("\n\n")
        .map(|it| it.to_owned())
        .collect::<Vec<_>>();

    let seeds = extract(&lines[0], "\\d+")
        .into_iter()
        .map(|it| it.parse().unwrap())
        .collect::<Vec<_>>();

    let mappers = lines
        .into_iter()
        .skip(1)
        .map(Mapper::from)
        .collect::<Vec<_>>();

    let destination = mappers
        .iter()
        .fold(seeds, |sources, mapper| mapper.find_destinations(sources))
        .into_iter()
        .min()
        .unwrap();

    println!("Part1: {}", destination)
}
