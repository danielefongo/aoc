use std::collections::{HashMap, HashSet};

use utils::{lines, read_input};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Dir {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos(i32, i32);
impl Pos {
    fn go_to(&self, dir: &Dir) -> Pos {
        match dir {
            Dir::North => Pos(self.0, self.1 - 1),
            Dir::South => Pos(self.0, self.1 + 1),
            Dir::West => Pos(self.0 - 1, self.1),
            Dir::East => Pos(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug)]
struct Elves {
    elves: HashSet<Pos>,
    preferred_dirs: Vec<Dir>,
    rounds: usize,
    completed: bool,
}
impl Elves {
    fn is_free(&self, from: &Pos, dir: &Dir) -> bool {
        let evaluating_dirs = match dir {
            Dir::North => [
                from.go_to(&Dir::North).go_to(&Dir::West),
                from.go_to(&Dir::North),
                from.go_to(&Dir::North).go_to(&Dir::East),
            ],
            Dir::South => [
                from.go_to(&Dir::South).go_to(&Dir::West),
                from.go_to(&Dir::South),
                from.go_to(&Dir::South).go_to(&Dir::East),
            ],
            Dir::West => [
                from.go_to(&Dir::West).go_to(&Dir::North),
                from.go_to(&Dir::West),
                from.go_to(&Dir::West).go_to(&Dir::South),
            ],
            Dir::East => [
                from.go_to(&Dir::East).go_to(&Dir::North),
                from.go_to(&Dir::East),
                from.go_to(&Dir::East).go_to(&Dir::South),
            ],
        };
        evaluating_dirs
            .into_iter()
            .all(|p| !self.elves.contains(&p))
    }
    fn has_neighbours(&self, from: &Pos) -> bool {
        [Dir::North, Dir::South, Dir::East, Dir::West]
            .into_iter()
            .any(|dir| !self.is_free(from, &dir))
    }
    fn no_movement_conflicts(&self, from: &Pos) -> bool {
        if let Some(dir) = self.preferred_dir(from) {
            let other_elf = from.go_to(dir).go_to(dir);
            self.preferred_pos(&other_elf) != self.preferred_pos(from)
        } else {
            true
        }
    }
    fn preferred_dir(&self, from: &Pos) -> Option<&Dir> {
        self.elves.get(from)?;
        self.preferred_dirs
            .iter()
            .find(|dir| self.is_free(from, dir))
    }
    fn preferred_pos(&self, from: &Pos) -> Pos {
        if !self.has_neighbours(from) {
            return from.clone();
        }

        self.preferred_dir(from)
            .map(|dir| from.go_to(dir))
            .unwrap_or(from.clone())
    }
    fn new_pos(&self, from: &Pos) -> Pos {
        if self.has_neighbours(from) && self.no_movement_conflicts(from) {
            self.preferred_pos(from)
        } else {
            from.clone()
        }
    }
    fn run(&mut self) {
        if self.completed {
            return;
        }

        let elves_pos = self.elves.iter().cloned().collect::<Vec<_>>();

        let mut movements: HashMap<Pos, Pos> = HashMap::new();

        for pos in elves_pos {
            let new_pos = self.new_pos(&pos);
            if new_pos != pos {
                movements.insert(pos.clone(), new_pos);
            }
        }

        if movements.is_empty() {
            self.completed = true;
        }

        movements.iter().for_each(|(old_pos, new_pos)| {
            self.elves.remove(old_pos);
            self.elves.insert(new_pos.clone());
        });

        self.rounds += 1;
        self.preferred_dirs.rotate_left(1);
    }
    fn value(&self) -> i32 {
        let min_x = self.elves.iter().map(|it| it.0).min().unwrap();
        let min_y = self.elves.iter().map(|it| it.1).min().unwrap();
        let max_x = self.elves.iter().map(|it| it.0).max().unwrap();
        let max_y = self.elves.iter().map(|it| it.1).max().unwrap();
        (max_y + 1 - min_y) * (max_x + 1 - min_x) - self.elves.len() as i32
    }
}
impl From<Vec<String>> for Elves {
    fn from(input: Vec<String>) -> Self {
        let mut elves: HashSet<Pos> = HashSet::new();
        input.iter().enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, c)| {
                if matches!(c, '#') {
                    elves.insert(Pos(x as i32, y as i32));
                }
            });
        });
        Self {
            elves,
            rounds: 0,
            completed: false,
            preferred_dirs: vec![Dir::North, Dir::South, Dir::West, Dir::East],
        }
    }
}

pub fn run() {
    let mut elves = Elves::from(lines(read_input!()));
    for _ in 0..10 {
        elves.run();
    }
    println!("Part1: {}", elves.value());

    while !elves.completed {
        elves.run();
    }
    println!("Part2: {}", elves.rounds);
}

#[cfg(test)]
mod tests {
    mod elves {
        use crate::day23::{Elves, Pos};

        #[test]
        fn build_elves() {
            let elves = Elves::from(vec![
                ".....".to_string(),
                "..##.".to_string(),
                "..#..".to_string(),
                ".....".to_string(),
                "..##.".to_string(),
                ".....".to_string(),
            ]);

            assert_eq!(
                elves.elves,
                [Pos(2, 1), Pos(3, 1), Pos(2, 2), Pos(2, 4), Pos(3, 4),]
                    .into_iter()
                    .collect()
            );
        }

        #[test]
        fn do_not_move_if_no_neighbours() {
            let mut elves = Elves::from(vec![
                "....".to_string(),
                ".#..".to_string(),
                "....".to_string(),
                "....".to_string(),
            ]);

            elves.run();
            assert_eq!(elves.elves, vec![Pos(1, 1)].into_iter().collect());
        }

        #[test]
        fn preferred_pos() {
            let elves = Elves::from(vec![
                "....".to_string(),
                ".#..".to_string(),
                ".#..".to_string(),
                "....".to_string(),
            ]);

            assert_eq!(elves.preferred_pos(&Pos(1, 1)), Pos(1, 0));
            assert_eq!(elves.preferred_pos(&Pos(1, 2)), Pos(1, 3));
        }

        #[test]
        fn move_elves_with_preferred_pos() {
            let mut elves = Elves::from(vec![
                "....".to_string(),
                ".#..".to_string(),
                ".#..".to_string(),
                "....".to_string(),
            ]);

            elves.run();
            assert_eq!(
                elves.elves,
                vec![Pos(1, 3), Pos(1, 0)].into_iter().collect()
            );
        }

        #[test]
        fn do_not_move_elves_with_preferred_pos() {
            let mut elves = Elves::from(vec![
                ".#..".to_string(),
                ".#..".to_string(),
                "....".to_string(),
                ".##.".to_string(),
            ]);

            elves.run();
            assert_eq!(
                elves.elves,
                vec![Pos(1, -1), Pos(1, 1), Pos(1, 3), Pos(2, 2)]
                    .into_iter()
                    .collect()
            );
        }
    }
}
