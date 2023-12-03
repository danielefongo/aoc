use utils::{extract, extract_one, lines, read_input};

#[derive(Debug)]
struct Run {
    reds: usize,
    blues: usize,
    greens: usize,
}
impl Run {
    fn allowed(&self, reds: usize, blues: usize, greens: usize) -> bool {
        self.reds <= reds && self.blues <= blues && self.greens <= greens
    }
}
impl From<String> for Run {
    fn from(value: String) -> Self {
        let blues = extract(&value, "\\d+ blue")
            .into_iter()
            .map(|val| extract_one(&val, "\\d+").parse::<usize>().unwrap())
            .sum();
        let reds = extract(&value, "\\d+ red")
            .into_iter()
            .map(|val| extract_one(&val, "\\d+").parse::<usize>().unwrap())
            .sum();
        let greens = extract(&value, "\\d+ green")
            .into_iter()
            .map(|val| extract_one(&val, "\\d+").parse::<usize>().unwrap())
            .sum();

        Self {
            reds,
            blues,
            greens,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    runs: Vec<Run>,
}
impl Game {
    fn allowed(&self, reds: usize, blues: usize, greens: usize) -> bool {
        self.runs.iter().all(|run| run.allowed(reds, blues, greens))
    }
}
impl From<String> for Game {
    fn from(value: String) -> Self {
        let id = extract_one(&value, "\\d+").parse().unwrap();
        let runs = value
            .replace("Game \\d+:", "")
            .split(';')
            .map(String::from)
            .map(Run::from)
            .collect();

        Self { id, runs }
    }
}

pub fn run() {
    let id_sum = lines(read_input!())
        .into_iter()
        .map(Game::from)
        .filter(|game| game.allowed(12, 14, 13))
        .map(|game| game.id)
        .sum::<usize>();

    println!("Part1: {}", id_sum)
}
