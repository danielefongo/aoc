use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Deref, DerefMut},
};

use utils::{lines, read_input};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
    Stay,
}
impl Dir {
    fn all() -> [Dir; 5] {
        [Dir::Up, Dir::Down, Dir::Right, Dir::Left, Dir::Stay]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Blizzards(u8);
impl Blizzards {
    fn add(&mut self, blizzards: Blizzards) {
        self.0 |= blizzards.0
    }
    fn contains(&mut self, blizzards: &Blizzards) -> bool {
        self.0 & blizzards.0 > 0
    }
    fn get_all_blizzards(&self) -> Vec<Blizzards> {
        [1, 2, 4, 8]
            .into_iter()
            .map(Blizzards)
            .filter(|it| self.clone().contains(it))
            .collect()
    }
    fn movement(&self) -> Dir {
        match self.0 {
            1 => Dir::Up,
            2 => Dir::Right,
            4 => Dir::Down,
            8 => Dir::Left,
            _ => panic!("invalid number"),
        }
    }
}
impl From<char> for Blizzards {
    fn from(c: char) -> Self {
        Self(match c {
            '^' => 1,
            '>' => 2,
            'v' => 4,
            '<' => 8,
            _ => 0,
        })
    }
}
impl Deref for Blizzards {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Blizzards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug)]
struct BlizzardsHistory {
    history: Vec<HashSet<Pos>>,
}
impl BlizzardsHistory {
    fn new(map: HashMap<Pos, Blizzards>, max_x: i32, max_y: i32) -> Self {
        let mut support: Vec<HashMap<Pos, Blizzards>> = Vec::new();
        let mut history: Vec<HashSet<Pos>> = Vec::new();
        support.push(map.clone());
        loop {
            let previous_map = support.last().unwrap().clone();
            let new_map = Self::move_blizzards(&previous_map, max_x, max_y);
            if support.contains(&new_map) {
                break;
            }
            support.push(new_map.clone());
            history.push(new_map.keys().cloned().collect());
        }
        support.remove(0);
        support.rotate_right(1);

        Self { history }
    }
    fn conflicting(&self, pos: &Pos, step: &usize) -> bool {
        let max = self.history.len();
        let candidates = self.history.get(step % max).unwrap();
        candidates.contains(pos)
    }
    fn forward(&mut self, steps: usize) {
        let history_size = self.history.len();
        self.history.rotate_left(steps % history_size)
    }
    fn move_blizzards(
        map: &HashMap<Pos, Blizzards>,
        max_x: i32,
        max_y: i32,
    ) -> HashMap<Pos, Blizzards> {
        let mut new_map: HashMap<Pos, Blizzards> = HashMap::new();

        map.iter()
            .map(|it| it.0)
            .flat_map(|pos| Self::move_blizzards_in(map, &pos, max_x, max_y))
            .for_each(|(new_pos, b)| {
                new_map
                    .entry(new_pos.clone())
                    .or_insert(Blizzards(0))
                    .add(b);
            });

        new_map
    }
    fn move_blizzards_in(
        map: &HashMap<Pos, Blizzards>,
        pos: &Pos,
        max_x: i32,
        max_y: i32,
    ) -> Vec<(Pos, Blizzards)> {
        if let Some(blizzards) = map.get(&pos) {
            blizzards
                .get_all_blizzards()
                .into_iter()
                .map(|v| (Self::blizzard_go_to(pos, &v.movement(), max_x, max_y), v))
                .collect()
        } else {
            vec![]
        }
    }
    fn blizzard_go_to(pos: &Pos, dir: &Dir, max_x: i32, max_y: i32) -> Pos {
        let warp = |x: i32, bound: i32| match (x, bound) {
            (x, bound) if x > bound => 1,
            (x, _) if x < 1 => bound,
            _ => x,
        };
        let new_pos = pos.go_to(dir);

        Pos(warp(new_pos.0, max_x), warp(new_pos.1, max_y))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);
impl Pos {
    fn go_to(&self, dir: &Dir) -> Pos {
        match dir {
            Dir::Up => Pos(self.0, self.1 - 1),
            Dir::Down => Pos(self.0, self.1 + 1),
            Dir::Right => Pos(self.0 + 1, self.1),
            Dir::Left => Pos(self.0 - 1, self.1),
            Dir::Stay => Pos(self.0, self.1),
        }
    }
    fn distance_from(&self, other: &Pos) -> usize {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
    }
}

#[derive(Clone, Debug)]
struct Step {
    pos: Pos,
    iterations: usize,
}

#[derive(Clone, Debug)]
struct Map {
    blizzards: BlizzardsHistory,
    actual: Pos,
    max_x: i32,
    max_y: i32,
    iterations: usize,
}
impl Map {
    fn go_to(&self, step: &Step, dir: &Dir) -> Option<Step> {
        let new_actual_pos = step.pos.go_to(dir);

        if (new_actual_pos.0 < 1
            || new_actual_pos.0 > self.max_x
            || new_actual_pos.1 < 1
            || new_actual_pos.1 > self.max_y)
            && new_actual_pos != Pos(1, 0)
            && new_actual_pos != Pos(self.max_x, self.max_y + 1)
        {
            return None;
        }

        if self
            .blizzards
            .conflicting(&new_actual_pos, &step.iterations)
        {
            return None;
        }

        Some(Step {
            pos: step.pos.go_to(dir),
            iterations: step.iterations + 1,
        })
    }
    fn reach(&mut self, pos: &Pos) {
        let mut best: Option<Step> = None;
        let mut best_iterations = usize::MAX;

        let mut queue: VecDeque<Step> = VecDeque::new();
        let mut maps_iterations: HashMap<(Pos, usize), usize> = HashMap::new();
        queue.push_front(Step {
            iterations: 0,
            pos: self.actual.clone(),
        });

        while let Some(step) = queue.pop_front() {
            if &step.pos == pos && step.iterations < best_iterations {
                best_iterations = step.iterations;
                best = Some(step);
                continue;
            }

            if step.iterations + step.pos.distance_from(pos) >= best_iterations {
                continue;
            }

            let description = (step.pos.clone(), step.iterations);
            let iterations_for_map = maps_iterations
                .get(&description)
                .unwrap_or(&usize::MAX)
                .clone();

            if step.iterations >= iterations_for_map {
                continue;
            }
            maps_iterations.insert(description, step.iterations);

            for dir in Dir::all() {
                if let Some(new_step) = self.go_to(&step, &dir) {
                    if new_step.pos.distance_from(pos) < step.pos.distance_from(pos) {
                        queue.push_front(new_step);
                    } else {
                        queue.push_back(new_step);
                    }
                }
            }
        }

        let last_step = best.unwrap().clone();
        self.blizzards.forward(last_step.iterations);
        self.iterations += last_step.iterations;
        self.actual = last_step.pos;
    }
}
impl From<Vec<String>> for Map {
    fn from(input: Vec<String>) -> Self {
        let actual = Pos(1, 0);
        let map = input
            .iter()
            .enumerate()
            .skip(1)
            .take(input.len() - 2)
            .flat_map(|(y, row)| {
                row.chars()
                    .into_iter()
                    .enumerate()
                    .skip(1)
                    .take(row.len() - 2)
                    .map(|(x, c)| (Pos(x as i32, y as i32), Blizzards::from(c)))
                    .collect::<Vec<(Pos, Blizzards)>>()
            })
            .collect::<HashMap<_, _>>();

        let max_x = map.keys().map(|it| it.0).max().unwrap() as i32;
        let max_y = map.keys().map(|it| it.1).max().unwrap() as i32;

        Self {
            blizzards: BlizzardsHistory::new(map, max_x, max_y),
            max_x,
            max_y,
            actual,
            iterations: 0,
        }
    }
}

pub fn run() {
    let mut map = Map::from(lines(read_input!()));

    let end = Pos(map.max_x, map.max_y + 1);
    let start = Pos(1, 0);

    map.reach(&end);
    println!("Part1: {}", map.iterations);
    map.reach(&start);
    map.reach(&end);
    println!("Part2: {}", map.iterations);
}
