use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use utils::read_input;

pub fn run() {
    let lines = read_input!().replace("\n", "");
    println!("Part1: {}", Cave::from(lines.clone()).solve(2022));
    println!("Part2: {}", Cave::from(lines.clone()).solve(1000000000000));
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
struct Pos(i64, i64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Movement {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct MovementGenerator {
    actual_movement: Option<Movement>,
    tick: bool,
    step: usize,
    movements: Vec<Movement>,
}
impl MovementGenerator {
    fn new(movements: Vec<Movement>) -> Self {
        Self {
            actual_movement: None,
            step: 0,
            tick: false,
            movements,
        }
    }
}
impl Iterator for MovementGenerator {
    type Item = Movement;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_) = self.actual_movement {
            if self.tick {
                self.actual_movement = Some(Movement::Down)
            } else {
                self.step = (self.step + 1) % self.movements.len();
                self.actual_movement = self.movements.get(self.step).map(Clone::clone);
            }
        } else {
            self.actual_movement = self.movements.get(self.step).map(Clone::clone);
        }

        self.tick = !self.tick;
        self.actual_movement
    }
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Row,
    Plus,
    L,
    Col,
    Square,
}
impl Iterator for Kind {
    type Item = Kind;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Kind::Row => Some(Kind::Plus),
            Kind::Plus => Some(Kind::L),
            Kind::L => Some(Kind::Col),
            Kind::Col => Some(Kind::Square),
            Kind::Square => Some(Kind::Row),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Block {
    kind: Kind,
    pos: Pos,
}
impl Block {
    fn new(pos: Pos, kind: Kind) -> Self {
        Self { pos, kind }
    }
    fn move_to(&self, movement: Movement) -> Self {
        match movement {
            Movement::Left => Self::new(Pos(self.pos.0 - 1, self.pos.1), self.kind),
            Movement::Right => Self::new(Pos(self.pos.0 + 1, self.pos.1), self.kind),
            Movement::Down => Self::new(Pos(self.pos.0, self.pos.1 - 1), self.kind),
        }
    }
    fn between(&self, from_x: i64, to_x: i64) -> bool {
        self.coordinates()
            .iter()
            .all(|it| it.0 >= from_x && it.0 <= to_x)
    }
    fn coordinates(&self) -> Vec<Pos> {
        match self.kind {
            Kind::Row => vec![
                Pos(self.pos.0, self.pos.1),
                Pos(self.pos.0 + 1, self.pos.1),
                Pos(self.pos.0 + 2, self.pos.1),
                Pos(self.pos.0 + 3, self.pos.1),
            ],
            Kind::Plus => vec![
                Pos(self.pos.0, self.pos.1 + 1),
                Pos(self.pos.0 + 1, self.pos.1),
                Pos(self.pos.0 + 1, self.pos.1 + 1),
                Pos(self.pos.0 + 1, self.pos.1 + 2),
                Pos(self.pos.0 + 2, self.pos.1 + 1),
            ],
            Kind::L => vec![
                Pos(self.pos.0, self.pos.1),
                Pos(self.pos.0 + 1, self.pos.1),
                Pos(self.pos.0 + 2, self.pos.1),
                Pos(self.pos.0 + 2, self.pos.1 + 1),
                Pos(self.pos.0 + 2, self.pos.1 + 2),
            ],
            Kind::Col => vec![
                Pos(self.pos.0, self.pos.1),
                Pos(self.pos.0, self.pos.1 + 1),
                Pos(self.pos.0, self.pos.1 + 2),
                Pos(self.pos.0, self.pos.1 + 3),
            ],
            Kind::Square => vec![
                Pos(self.pos.0, self.pos.1),
                Pos(self.pos.0, self.pos.1 + 1),
                Pos(self.pos.0 + 1, self.pos.1),
                Pos(self.pos.0 + 1, self.pos.1 + 1),
            ],
        }
    }
}

#[derive(Debug)]
struct Spawner {
    actual_kind: Kind,
}
impl Spawner {
    fn new() -> Self {
        Self {
            actual_kind: Kind::Square,
        }
    }
    fn generate(&mut self, pos: Pos) -> Block {
        self.actual_kind = self.actual_kind.next().unwrap();
        Block::new(pos, self.actual_kind)
    }
}

#[derive(Debug)]
struct Cave {
    actual_block: Block,
    generator: MovementGenerator,
    spawner: Spawner,
    max_height: i64,
    inserted_blocks: i64,
    occupied: HashSet<Pos>,
    iterations: u64,
    blocks: Vec<(usize, Block)>,
    windows: HashMap<[(usize, i64, i64, Kind); 5], (i64, i64, Vec<Block>)>,
    shortcut_found: bool,
    target: i64,
}
impl From<String> for Cave {
    fn from(input: String) -> Self {
        let movements = input
            .chars()
            .map(|it| match it {
                '>' => Movement::Right,
                '<' => Movement::Left,
                'v' => Movement::Down,
                _ => panic!("invalid input"),
            })
            .collect();
        let generator = MovementGenerator::new(movements);
        Self::new(generator, 0)
    }
}
impl Cave {
    fn new(generator: MovementGenerator, target: i64) -> Self {
        let mut spawner = Spawner::new();

        let mut cave = Self {
            generator,
            actual_block: spawner.generate(Pos(3, 4)),
            spawner,
            max_height: 0,
            inserted_blocks: 0,
            occupied: HashSet::new(),
            iterations: 0,
            blocks: Vec::new(),
            windows: HashMap::new(),
            shortcut_found: false,
            target,
        };

        cave.insert_row();
        cave
    }
    fn solve(&mut self, target: i64) -> usize {
        self.target = target;
        while self.shortcut_found == false {
            self.iter();
        }
        while self.inserted_blocks < self.target {
            self.iter();
        }
        self.max_height as usize
    }
    fn iter(&mut self) {
        let movement = &self.generator.next();
        self.iterations += 1;
        self.execute_movement(movement.unwrap());
    }
    fn execute_movement(&mut self, movement: Movement) {
        let candidate = self.actual_block.move_to(movement);

        if !candidate.between(1, 7) {
            return;
        }

        if !self.already_occupied(candidate) {
            return self.actual_block = candidate;
        }

        if movement == Movement::Down {
            self.add_new_block()
        }
    }
    fn add_new_block(&mut self) {
        self.actual_block.coordinates().into_iter().for_each(|p| {
            self.max_height = self.max_height.max(p.1);
            self.occupied.insert(p);
        });
        self.inserted_blocks += 1;

        self.blocks
            .push((self.generator.step, self.actual_block.clone()));

        self.actual_block = self.spawner.generate(Pos(3, self.max_height + 4));

        if self.inserted_blocks > 5 {
            let last_five: Vec<(usize, Block)> =
                self.blocks.iter().rev().take(5).rev().cloned().collect();

            let actual_blocks: Vec<Block> = self
                .blocks
                .iter()
                .rev()
                .take(5)
                .rev()
                .map(|(_, b)| b.clone())
                .collect();

            let window = [
                (last_five[0].0, 0, last_five[0].1.pos.0, last_five[0].1.kind),
                (
                    last_five[1].0,
                    last_five[1].1.pos.1 - last_five[0].1.pos.1,
                    last_five[1].1.pos.0,
                    last_five[1].1.kind,
                ),
                (
                    last_five[2].0,
                    last_five[2].1.pos.1 - last_five[1].1.pos.1,
                    last_five[2].1.pos.0,
                    last_five[2].1.kind,
                ),
                (
                    last_five[3].0,
                    last_five[3].1.pos.1 - last_five[2].1.pos.1,
                    last_five[3].1.pos.0,
                    last_five[3].1.kind,
                ),
                (
                    last_five[4].0,
                    last_five[4].1.pos.1 - last_five[3].1.pos.1,
                    last_five[4].1.pos.0,
                    last_five[4].1.kind,
                ),
            ];

            if self.windows.contains_key(&window) && !self.shortcut_found {
                let (initial_blocks, initial_height, previous_blocks) =
                    self.windows.get(&window).unwrap().clone();

                let skip_blocks = self.inserted_blocks - initial_blocks;
                let skip_height = self.max_height - initial_height;

                let count = (self.target - initial_blocks) / skip_blocks;

                self.max_height = initial_height + skip_height * count;
                self.inserted_blocks = initial_blocks + skip_blocks * count;

                previous_blocks.iter().for_each(|b| {
                    b.coordinates().iter().for_each(|p| {
                        self.occupied.insert(Pos(p.0, p.1 + skip_height * count));
                    })
                });

                self.actual_block.pos.1 = self.max_height + 4;

                self.shortcut_found = true;
            }

            self.windows.insert(
                window,
                (self.inserted_blocks, self.max_height, actual_blocks),
            );
        }
    }
    fn already_occupied(&self, candidate: Block) -> bool {
        candidate
            .coordinates()
            .iter()
            .any(|it| self.occupied.contains(it))
    }
    fn insert_row(&mut self) {
        (1..=7).for_each(|x| {
            self.occupied.insert(Pos(x, self.max_height));
        });
    }
}

#[cfg(test)]
mod test {
    mod block {
        use crate::day17::{Block, Kind, Movement, Pos};

        #[test]
        fn row_block_creation() {
            let block = Block::new(Pos(3, 3), Kind::Row);
            assert_eq!(
                block.coordinates(),
                vec![Pos(3, 3), Pos(4, 3), Pos(5, 3), Pos(6, 3)]
            );
        }

        #[test]
        fn plus_block_creation() {
            let block = Block::new(Pos(3, 3), Kind::Plus);
            assert_eq!(
                block.coordinates(),
                vec![Pos(3, 4), Pos(4, 3), Pos(4, 4), Pos(4, 5), Pos(5, 4)]
            );
        }

        #[test]
        fn l_block_creation() {
            let block = Block::new(Pos(3, 3), Kind::L);
            assert_eq!(
                block.coordinates(),
                vec![Pos(3, 3), Pos(4, 3), Pos(5, 3), Pos(5, 4), Pos(5, 5)]
            );
        }

        #[test]
        fn col_block_creation() {
            let block = Block::new(Pos(3, 3), Kind::Col);
            assert_eq!(
                block.coordinates(),
                vec![Pos(3, 3), Pos(3, 4), Pos(3, 5), Pos(3, 6)]
            );
        }

        #[test]
        fn square_block_creation() {
            let block = Block::new(Pos(3, 3), Kind::Square);
            assert_eq!(
                block.coordinates(),
                vec![Pos(3, 3), Pos(3, 4), Pos(4, 3), Pos(4, 4)]
            );
        }

        #[test]
        fn move_block() {
            let block = Block::new(Pos(3, 3), Kind::Square);
            assert_eq!(
                block.move_to(Movement::Left).coordinates(),
                vec![Pos(2, 3), Pos(2, 4), Pos(3, 3), Pos(3, 4)]
            );
            assert_eq!(
                block.move_to(Movement::Right).coordinates(),
                vec![Pos(4, 3), Pos(4, 4), Pos(5, 3), Pos(5, 4)]
            );
            assert_eq!(
                block.move_to(Movement::Down).coordinates(),
                vec![Pos(3, 2), Pos(3, 3), Pos(4, 2), Pos(4, 3)]
            );
        }
    }

    mod spawner {
        use crate::day17::{Kind, Pos, Spawner};

        #[test]
        fn spawn_properly() {
            let mut spawner = Spawner::new();
            assert_eq!(spawner.generate(Pos(0, 0)).kind, Kind::Row);
            assert_eq!(spawner.generate(Pos(0, 0)).kind, Kind::Plus);
            assert_eq!(spawner.generate(Pos(0, 0)).kind, Kind::L);
            assert_eq!(spawner.generate(Pos(0, 0)).kind, Kind::Col);
            assert_eq!(spawner.generate(Pos(0, 0)).kind, Kind::Square);
            assert_eq!(spawner.generate(Pos(0, 0)).kind, Kind::Row);
        }
    }

    mod movement_generator {
        use crate::day17::{Movement, MovementGenerator};

        #[test]
        fn rotate_movements() {
            let mut generator = MovementGenerator::new(vec![Movement::Left, Movement::Right]);
            assert_eq!(generator.next(), Some(Movement::Left));
            assert_eq!(generator.next(), Some(Movement::Down));
            assert_eq!(generator.next(), Some(Movement::Right));
            assert_eq!(generator.next(), Some(Movement::Down));
            assert_eq!(generator.next(), Some(Movement::Left));
        }
    }

    mod cave {
        use crate::{
            day17::{Block, Cave, Kind, Pos},
            utils::{lines, read_input},
        };

        #[test]
        fn generate_cave() {
            let cave = Cave::from("<>".to_string());

            assert_eq!(cave.actual_block, Block::new(Pos(3, 4), Kind::Row));
        }

        #[test]
        fn iter_without_obstacles() {
            let mut cave = Cave::from("<".to_string());

            cave.iter();
            cave.iter();
            cave.iter();
            assert_eq!(cave.actual_block, Block::new(Pos(1, 3), Kind::Row));
        }

        #[test]
        fn iter_until_wall() {
            let mut cave = Cave::from("<".to_string());

            cave.iter();
            cave.iter();
            cave.iter();
            cave.iter();
            assert_eq!(cave.actual_block, Block::new(Pos(1, 2), Kind::Row));
            cave.iter();
            assert_eq!(cave.actual_block, Block::new(Pos(1, 2), Kind::Row));
            cave.iter();
            assert_eq!(cave.actual_block, Block::new(Pos(1, 1), Kind::Row));
        }

        #[test]
        fn iter_until_floor() {
            let mut cave = Cave::from("v".to_string());

            cave.iter();
            cave.iter();
            cave.iter();
            assert_eq!(cave.actual_block, Block::new(Pos(3, 1), Kind::Row));
            cave.iter();
            assert_eq!(cave.actual_block, Block::new(Pos(3, 5), Kind::Plus));
        }

        #[test]
        fn iter_until_another_block() {
            let mut cave = Cave::from("<".to_string());

            cave.occupied.insert(Pos(2, 4));

            cave.iter();

            assert_eq!(cave.actual_block, Block::new(Pos(3, 4), Kind::Row));
        }

        #[test]
        fn iter_multiple_blocks() {
            let mut cave = Cave::from("v".to_string());

            cave.iter();
            cave.iter();
            cave.iter();
            cave.iter();

            assert_eq!(cave.actual_block, Block::new(Pos(3, 5), Kind::Plus));

            cave.iter();
            cave.iter();
            cave.iter();
            cave.iter();

            assert_eq!(cave.actual_block, Block::new(Pos(3, 8), Kind::L));
        }

        #[test]
        fn part1() {
            let mut cave = Cave::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string());
            assert_eq!(cave.solve(2022), 3068);
        }

        #[test]
        fn part2() {
            let mut cave = Cave::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string());
            assert_eq!(cave.solve(1000000000000), 1514285714288);
        }
    }
}
