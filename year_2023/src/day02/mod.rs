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
    fn generate_max_run(&self) -> Run {
        let blues = self
            .runs
            .iter()
            .map(|it| it.blues)
            .filter(|it| it > &0)
            .max()
            .unwrap_or_default();
        let reds = self
            .runs
            .iter()
            .map(|it| it.reds)
            .filter(|it| it > &0)
            .max()
            .unwrap_or_default();
        let greens = self
            .runs
            .iter()
            .map(|it| it.greens)
            .filter(|it| it > &0)
            .max()
            .unwrap_or_default();

        Run {
            reds,
            blues,
            greens,
        }
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
    let games = lines(read_input!())
        .into_iter()
        .map(Game::from)
        .collect::<Vec<_>>();

    println!(
        "Part1: {}",
        games
            .iter()
            .filter(|game| game.allowed(12, 14, 13))
            .map(|game| game.id)
            .sum::<usize>()
    );

    println!(
        "Part2: {}",
        games
            .iter()
            .map(|game| game.generate_max_run())
            .map(|run| run.greens * run.blues * run.reds)
            .sum::<usize>()
    );
}
