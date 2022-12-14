use std::{fmt::Display, isize};

use crate::utils::{lines, read_input};

type Pos = (isize, isize);

#[derive(Debug, PartialEq, Eq)]
struct Boundary {
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}
impl Boundary {
    fn new(top_left: Pos, bottom_right: Pos) -> Self {
        Self {
            min_x: top_left.0,
            min_y: top_left.1,
            max_x: bottom_right.0,
            max_y: bottom_right.1,
        }
    }
    fn contains(&self, pos: Pos) -> bool {
        pos.0 >= self.min_x && pos.0 <= self.max_x && pos.1 >= self.min_y && pos.1 <= self.max_y
    }
    fn relative(&self, pos: Pos) -> Pos {
        (pos.0 - self.min_x, pos.1 - self.min_y)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Path {
    actual: Option<Pos>,
    start: Pos,
    end: Pos,
}
impl Path {
    fn new(start: Pos, end: Pos) -> Self {
        let (start, end) = if start.0 > end.0 || start.1 > end.1 {
            (end, start)
        } else {
            (start, end)
        };

        Self {
            start,
            end,
            actual: None,
        }
    }
}
impl From<Vec<String>> for Path {
    fn from(input: Vec<String>) -> Self {
        let (start_x, start_y) = input[0].split_once(",").unwrap();
        let (end_x, end_y) = input[1].split_once(",").unwrap();

        let start = (start_x.parse().unwrap(), start_y.parse().unwrap());
        let end = (end_x.parse().unwrap(), end_y.parse().unwrap());

        Self::new(start, end)
    }
}
impl Iterator for Path {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let actual = if let Some(mut actual) = self.actual {
            if self.start.0 == self.end.0 && actual.1 < self.end.1 {
                actual.1 += 1;
            } else if self.start.1 == self.end.1 && actual.0 < self.end.0 {
                actual.0 += 1;
            } else {
                return None;
            }
            actual
        } else {
            self.start
        };
        self.actual = Some(actual);
        self.actual
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Block {
    Sand,
    Rock,
    Void,
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Block::Sand => "o",
            Block::Rock => "#",
            Block::Void => ".",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug)]
struct Cave {
    spawn: Pos,
    boundary: Boundary,
    actual_sand_pos: Pos,
    blocks: Vec<Vec<Option<Block>>>,
    sands: usize,
}
impl Cave {
    fn at(&self, pos: Pos) -> Option<Block> {
        if !self.boundary.contains(pos) {
            return None;
        }

        let (x, y) = self.boundary.relative(pos);

        self.blocks[x as usize][y as usize].clone()
    }
    fn set(&mut self, pos: Pos, block: Block) {
        if self.boundary.contains(pos) {
            let (x, y) = self.boundary.relative(pos);
            self.blocks[x as usize][y as usize] = Some(block);
        }
    }
    fn run(&mut self) -> usize {
        while let Some(()) = self.do_step() {}
        self.set(self.actual_sand_pos, Block::Void);
        self.sands
    }
    fn do_step(&mut self) -> Option<()> {
        let moved = self.try_bottom()? || self.try_bottom_left()? || self.try_bottom_right()?;

        if !moved {
            self.actual_sand_pos = self.spawn;
            self.set(self.actual_sand_pos, Block::Sand);
            self.sands += 1;
        }

        Some(())
    }
    fn try_bottom(&mut self) -> Option<bool> {
        self.try_move((self.actual_sand_pos.0, self.actual_sand_pos.1 + 1))
    }
    fn try_bottom_left(&mut self) -> Option<bool> {
        self.try_move((self.actual_sand_pos.0 - 1, self.actual_sand_pos.1 + 1))
    }
    fn try_bottom_right(&mut self) -> Option<bool> {
        self.try_move((self.actual_sand_pos.0 + 1, self.actual_sand_pos.1 + 1))
    }
    fn try_move(&mut self, candidate_pos: Pos) -> Option<bool> {
        match self.at(candidate_pos)? {
            Block::Void => {
                self.set(candidate_pos, Block::Sand);
                self.set(self.actual_sand_pos, Block::Void);
                self.actual_sand_pos = candidate_pos;
                Some(true)
            }
            _ => Some(false),
        }
    }
}
impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blocks = (self.boundary.min_y..=self.boundary.max_y)
            .map(|y| {
                (self.boundary.min_x..=self.boundary.max_x)
                    .map(|x| self.at((x, y)).unwrap().to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", blocks)
    }
}
impl From<Vec<Path>> for Cave {
    fn from(paths: Vec<Path>) -> Self {
        let spawn = (500, 0);

        let mut min_x = isize::MAX;
        let min_y = 0;
        let mut max_x = 500;
        let mut max_y = 0;

        paths.iter().for_each(|it| {
            let min = it.min().unwrap();
            let max = it.max().unwrap();
            min_x = if min.0 < min_x { min.0 } else { min_x };
            max_x = if max.0 > max_x { max.0 } else { max_x };
            max_y = if max.1 > max_y { max.1 } else { max_y };
        });

        let blocks = (min_x..=max_x)
            .map(|_| (min_y..=max_y).map(|_| Some(Block::Void)).collect())
            .collect();

        let mut cave = Self {
            blocks,
            actual_sand_pos: spawn,
            spawn,
            boundary: Boundary::new((min_x, min_y), (max_x, max_y)),
            sands: 0,
        };

        paths.clone().iter_mut().for_each(|it| {
            while let Some(pos) = it.next() {
                cave.set(pos, Block::Rock);
            }
        });
        cave.set(cave.actual_sand_pos, Block::Sand);

        cave
    }
}

pub fn run() {
    let mut cave = create_cave(lines(read_input(14)));
    println!("Part1: {}", cave.run());
    println!("{}", cave);
}

fn create_cave(lines: Vec<String>) -> Cave {
    lines
        .iter()
        .map(|it| {
            let a: Vec<String> = it.split(" -> ").map(|it| it.to_string()).collect();
            a.windows(2)
                .map(|it| it.to_vec().into())
                .collect::<Vec<Path>>()
        })
        .flatten()
        .collect::<Vec<Path>>()
        .into()
}

#[cfg(test)]
mod tests {
    mod boundary {
        use crate::day14::Boundary;

        #[test]
        fn contains() {
            let boundary = Boundary::new((0, 0), (2, 2));
            assert_eq!(boundary.contains((0, 0)), true);
            assert_eq!(boundary.contains((0, 2)), true);
            assert_eq!(boundary.contains((2, 0)), true);
            assert_eq!(boundary.contains((2, 2)), true);
        }

        #[test]
        fn relative() {
            let boundary = Boundary::new((1, 1), (2, 2));
            assert_eq!(boundary.relative((0, 0)), (-1, -1));
            assert_eq!(boundary.relative((3, 3)), (2, 2));
        }
    }

    mod path {
        use crate::day14::Path;

        #[test]
        fn parse_path() {
            let expected_path = Path::new((0, 0), (0, 2));
            let path: Path = vec!["0,0".to_string(), "0,2".to_string()].into();

            assert_eq!(path, expected_path);
        }

        #[test]
        fn iter_on_vertical_path() {
            let mut path = Path::new((2, 3), (2, 6));
            assert_eq!(path.next(), Some((2, 3)));
            assert_eq!(path.next(), Some((2, 4)));
            assert_eq!(path.next(), Some((2, 5)));
            assert_eq!(path.next(), Some((2, 6)));
            assert_eq!(path.next(), None);
        }

        #[test]
        fn iter_on_horizontal_path() {
            let mut path = Path::new((3, 2), (6, 2));
            assert_eq!(path.next(), Some((3, 2)));
            assert_eq!(path.next(), Some((4, 2)));
            assert_eq!(path.next(), Some((5, 2)));
            assert_eq!(path.next(), Some((6, 2)));
            assert_eq!(path.next(), None);
        }

        #[test]
        fn iter_on_reversed_vertical_path() {
            let mut path = Path::new((2, 6), (2, 3));
            assert_eq!(path.next(), Some((2, 3)));
            assert_eq!(path.next(), Some((2, 4)));
            assert_eq!(path.next(), Some((2, 5)));
            assert_eq!(path.next(), Some((2, 6)));
            assert_eq!(path.next(), None);
        }

        #[test]
        fn iter_on_reversed_horizontal_path() {
            let mut path = Path::new((6, 2), (3, 2));
            assert_eq!(path.next(), Some((3, 2)));
            assert_eq!(path.next(), Some((4, 2)));
            assert_eq!(path.next(), Some((5, 2)));
            assert_eq!(path.next(), Some((6, 2)));
            assert_eq!(path.next(), None);
        }

        #[test]
        fn max_of_path() {
            assert_eq!(Path::new((0, 0), (0, 2)).max().unwrap(), (0, 2));
        }
    }

    mod cave {
        use crate::{
            day14::{create_cave, Block, Boundary, Cave, Path},
            utils::lines,
        };

        #[test]
        fn parse_cave() {
            let paths = vec![Path::new((498, 4), (498, 6))];
            let cave: Cave = paths.into();
            assert_eq!(cave.spawn, (500, 0));
            assert_eq!(cave.boundary, Boundary::new((498, 0), (500, 6)));
        }

        #[test]
        fn find_at() {
            let paths = vec![Path::new((498, 4), (498, 6))];
            let cave: Cave = paths.into();
            assert_eq!(cave.at((498, 4)), Some(Block::Rock));
            assert_eq!(cave.at((498, 5)), Some(Block::Rock));
            assert_eq!(cave.at((498, 6)), Some(Block::Rock));
            assert_eq!(cave.at((498, 6)), Some(Block::Rock));
            assert_eq!(cave.at((500, 0)), Some(Block::Sand));
            assert_eq!(cave.at((500, 1)), Some(Block::Void));
            assert_eq!(cave.at((501, 0)), None);
            assert_eq!(cave.at((497, 0)), None);
            assert_eq!(cave.at((500, -1)), None);
            assert_eq!(cave.at((500, 7)), None);
        }

        #[test]
        fn set_at() {
            let paths = vec![Path::new((498, 4), (498, 6))];
            let mut cave: Cave = paths.into();
            cave.set((500, 1), Block::Sand);
            assert_eq!(cave.at((500, 1)), Some(Block::Sand));
        }

        #[test]
        fn move_sand_bottom() {
            let paths = vec![Path::new((498, 2), (502, 2))];
            let mut cave: Cave = paths.into();
            assert_eq!(cave.try_bottom(), Some(true));
            assert_eq!(cave.try_bottom(), Some(false));
            assert_eq!(cave.at((500, 0)), Some(Block::Void));
            assert_eq!(cave.at((500, 1)), Some(Block::Sand));
        }

        #[test]
        fn move_sand_bottom_left() {
            let paths = vec![Path::new((498, 2), (502, 2)), Path::new((500, 1), (500, 1))];
            let mut cave: Cave = paths.into();
            assert_eq!(cave.try_bottom_left(), Some(true));
            assert_eq!(cave.try_bottom_left(), Some(false));
            assert_eq!(cave.at((500, 0)), Some(Block::Void));
            assert_eq!(cave.at((499, 1)), Some(Block::Sand));
        }

        #[test]
        fn move_sand_bottom_right() {
            let paths = vec![Path::new((498, 2), (502, 2)), Path::new((500, 1), (500, 1))];
            let mut cave: Cave = paths.into();
            assert_eq!(cave.try_bottom_right(), Some(true));
            assert_eq!(cave.try_bottom_right(), Some(false));
            assert_eq!(cave.at((500, 0)), Some(Block::Void));
            assert_eq!(cave.at((501, 1)), Some(Block::Sand));
        }

        #[test]
        fn fall_outside() {
            let paths = vec![Path::new((498, 1), (498, 1))];
            let mut cave: Cave = paths.into();
            assert_eq!(cave.try_bottom(), Some(true));
            assert_eq!(cave.try_bottom(), None);
        }

        #[test]
        fn part1() {
            let lines =
                lines("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n".to_string());

            let mut cave = create_cave(lines);
            assert_eq!(cave.run(), 24);
        }
    }
}
