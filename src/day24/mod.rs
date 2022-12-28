use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use crate::utils::{lines, read_input};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
    Stay,
}
impl Dir {
    fn all() -> Vec<Dir> {
        vec![Dir::Up, Dir::Down, Dir::Right, Dir::Left, Dir::Stay]
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
}

#[derive(Clone, Debug)]
struct Map {
    blizzards: HashMap<Pos, u8>,
    actual: Pos,
    max_x: i32,
    max_y: i32,
    iterations: usize,
}
impl Map {
    fn new(map: HashMap<Pos, u8>, actual: Pos, max_x: i32, max_y: i32, iterations: usize) -> Self {
        Self {
            blizzards: map,
            actual,
            max_x,
            max_y,
            iterations,
        }
    }
    fn display_value(value: u8) -> String {
        match value {
            0 => " ",
            1 => "^",
            2 => ">",
            3 => "2",
            4 => "v",
            5 => "2",
            6 => "2",
            7 => "3",
            8 => "<",
            9 => "2",
            10 => "2",
            11 => "3",
            12 => "2",
            13 => "3",
            14 => "3",
            15 => "4",
            _ => panic!("invalid value"),
        }
        .to_string()
    }
    fn get_blizzards_from(&self, value: u8) -> Vec<u8> {
        [1, 2, 4, 8]
            .into_iter()
            .map(|it| value & it)
            .filter(|it| it > &0)
            .collect()
    }
    fn warp(&self, pos: Pos) -> Pos {
        let rotate = |x: i32, bound: i32| match (x, bound) {
            (x, bound) if x > bound => 1,
            (x, _) if x < 1 => bound,
            _ => x,
        };

        Pos(rotate(pos.0, self.max_x), rotate(pos.1, self.max_y))
    }
    fn go_to(&self, dir: &Dir) -> Option<Map> {
        let new_actual_pos = self.actual.go_to(dir);

        if (new_actual_pos.0 < 1
            || new_actual_pos.0 > self.max_x
            || new_actual_pos.1 < 1
            || new_actual_pos.1 > self.max_y)
            && new_actual_pos != Pos(1, 0)
            && new_actual_pos != Pos(self.max_x, self.max_y + 1)
        {
            return None;
        }

        let mut new_map = self.move_blizzards();
        if new_map.blizzards.get(&new_actual_pos).unwrap_or(&0) != &0 {
            return None;
        }

        new_map.actual = self.actual.go_to(dir);
        Some(new_map)
    }
    fn movement_for_blizzard(&self, pos: Pos, value: u8) -> Pos {
        let new_pos = match value {
            1 => pos.go_to(&Dir::Up),
            2 => pos.go_to(&Dir::Right),
            4 => pos.go_to(&Dir::Down),
            8 => pos.go_to(&Dir::Left),
            _ => panic!("invalid number"),
        };
        self.warp(new_pos)
    }
    fn move_blizzards(&self) -> Map {
        let positions: Vec<_> = self.blizzards.iter().map(|it| it.0).cloned().collect();
        let mut new_map: HashMap<Pos, u8> = HashMap::new();

        for pos in positions {
            let value = self.blizzards.get(&pos).unwrap().clone();
            for v in self.get_blizzards_from(value) {
                let new_pos = self.movement_for_blizzard(pos.clone(), v);

                *new_map.entry(new_pos).or_insert(0) |= v;
            }
        }

        Map::new(
            new_map,
            self.actual.clone(),
            self.max_x,
            self.max_y,
            self.iterations + 1,
        )
    }
    fn reach(&self, pos: &Pos) -> Map {
        let mut best: Option<Map> = None;
        let mut best_iterations = usize::MAX;

        let mut queue: VecDeque<Map> = VecDeque::new();
        let mut maps_iterations: HashMap<String, usize> = HashMap::new();
        queue.push_front(self.clone());

        while let Some(map) = queue.pop_front() {
            if &map.actual == pos && map.iterations < best_iterations {
                best_iterations = map.iterations;
                best = Some(map);
                continue;
            }

            let description = format!("{}", map);
            let iterations_for_map = maps_iterations
                .get(&description)
                .unwrap_or(&usize::MAX)
                .clone();

            if map.iterations >= iterations_for_map && &map.actual != pos {
                continue;
            }

            if map.iterations > best_iterations {
                continue;
            }

            maps_iterations.insert(description, map.iterations);

            for dir in Dir::all() {
                if let Some(new_map) = map.go_to(&dir) {
                    queue.push_back(new_map);
                }
            }
        }

        best.unwrap().clone()
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_line: String = (0..=(self.max_x + 1))
            .map(|_| "#".to_string())
            .collect::<Vec<_>>()
            .join("");

        let lines = (1..=self.max_y)
            .map(|y| {
                let row = (1..=self.max_x)
                    .map(|x| {
                        let pos = Pos(x, y);
                        let value = self.blizzards.get(&pos).unwrap_or(&0);
                        if pos == self.actual {
                            "O".to_string()
                        } else {
                            Self::display_value(value.clone())
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("");
                format!("#{}#", row).to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}\n{}\n{}", first_line, lines, first_line)
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
                    .map(|(x, c)| {
                        let pos = Pos(x as i32, y as i32);
                        let value = match c {
                            '^' => 1,
                            '>' => 2,
                            'v' => 4,
                            '<' => 8,
                            _ => 0,
                        };
                        (pos, value)
                    })
                    .collect::<Vec<(Pos, u8)>>()
            })
            .collect::<HashMap<_, _>>();

        let max_x = map.keys().map(|it| it.0).max().unwrap() as i32;
        let max_y = map.keys().map(|it| it.1).max().unwrap() as i32;
        Self {
            actual,
            blizzards: map,
            max_x,
            max_y,
            iterations: 0,
        }
    }
}

pub fn run() {
    let map = Map::from(lines(read_input(24)));

    let end = Pos(map.max_x, map.max_y + 1);
    let start = Pos(1, 0);

    let map = map.reach(&end);
    println!("Part1: {}", map.iterations);
    let map = map.reach(&start).reach(&end);
    println!("Part2: {}", map.iterations);
}

#[cfg(test)]
mod tests {
    use crate::day24::{Dir, Pos};

    use super::Map;

    #[test]
    fn movements() {
        let map = Map::from(vec![
            "# ###".to_string(),
            "#> <#".to_string(),
            "### #".to_string(),
        ]);

        assert_eq!(map.blizzards.get(&Pos(1, 1)), Some(&2));
        assert_eq!(map.blizzards.get(&Pos(2, 1)), Some(&0));
        assert_eq!(map.blizzards.get(&Pos(3, 1)), Some(&8));
        let map = map.move_blizzards();
        assert_eq!(map.blizzards.get(&Pos(1, 1)), None);
        assert_eq!(map.blizzards.get(&Pos(2, 1)), Some(&10));
        assert_eq!(map.blizzards.get(&Pos(3, 1)), None);
    }

    #[test]
    fn movements_over_walls() {
        let map = Map::from(vec![
            "# ###".to_string(),
            "#  >#".to_string(),
            "# v #".to_string(),
            "### #".to_string(),
        ]);

        assert_eq!(map.blizzards.get(&Pos(3, 1)), Some(&2));
        assert_eq!(map.blizzards.get(&Pos(2, 2)), Some(&4));
        let map = map.move_blizzards();
        assert_eq!(map.blizzards.get(&Pos(1, 1)), Some(&2));
        assert_eq!(map.blizzards.get(&Pos(2, 1)), Some(&4));
    }

    #[test]
    fn cannot_move_over_actual_pos() {
        let mut map = Map::from(vec![
            "# ###".to_string(),
            "#E< #".to_string(),
            "#   #".to_string(),
            "### #".to_string(),
        ]);

        map.actual = Pos(1, 1);
        assert_eq!(map.go_to(&Dir::Stay).is_none(), true);
    }

    #[test]
    fn cannot_move() {
        let map = Map::from(vec![
            "# ###".to_string(),
            "# < #".to_string(),
            "#   #".to_string(),
            "### #".to_string(),
        ]);

        assert_eq!(map.go_to(&Dir::Down).is_some(), false);
        assert_eq!(map.go_to(&Dir::Right).is_some(), false);
        assert_eq!(map.go_to(&Dir::Up).is_some(), false);
        assert_eq!(map.go_to(&Dir::Down).is_some(), false);
        assert_eq!(map.go_to(&Dir::Down).is_some(), false);
        let map = map.go_to(&Dir::Stay).unwrap();
        assert_eq!(map.go_to(&Dir::Down).is_some(), true);
    }
}
