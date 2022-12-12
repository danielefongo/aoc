use std::collections::HashMap;

use crate::utils::read_input;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn nearest(&self) -> Vec<Pos> {
        vec![
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x, self.y + 1),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum PalaceType {
    S,
    E,
    N,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Palace {
    type_: PalaceType,
    pos: Pos,
    height: u8,
    distance: usize,
}
impl Palace {
    fn new(type_: PalaceType, pos: Pos, height: u8) -> Self {
        Self {
            type_,
            pos,
            height,
            distance: 0,
        }
    }
    fn can_go_to(&self, other: &Palace) -> bool {
        other.height <= self.height + 1
    }
}

#[derive(Debug)]
struct Elevations {
    palaces: HashMap<Pos, Palace>,
}
impl From<String> for Elevations {
    fn from(input: String) -> Self {
        Self {
            palaces: input
                .split("\n")
                .enumerate()
                .map(|(r_idx, row)| {
                    row.chars()
                        .enumerate()
                        .map(|(c_idx, palace)| {
                            let pos = Pos::new(c_idx as i32, r_idx as i32);
                            let pos2 = pos.clone();
                            let palace = match palace {
                                'S' => Palace::new(PalaceType::S, pos, 0),
                                'E' => Palace::new(PalaceType::E, pos, 25),
                                it => Palace::new(PalaceType::N, pos, (it as u8) - ('a' as u8)),
                            };
                            (pos2, palace)
                        })
                        .collect::<Vec<(Pos, Palace)>>()
                })
                .flatten()
                .collect(),
        }
    }
}
impl Elevations {
    fn run(
        &self,
        is_start: fn(&Palace) -> bool,
        is_end: fn(&Palace) -> bool,
        can_go_to: fn(&Palace, &Palace) -> bool,
    ) -> usize {
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        let mut queue: HashMap<Pos, &Palace> = HashMap::new();

        for palace in self.palaces.values() {
            if is_start(palace) {
                distances.insert(palace.pos.clone(), 0);
            } else {
                distances.insert(palace.pos.clone(), usize::MAX);
            }

            queue.insert(palace.pos.clone(), palace);
        }

        while !queue.is_empty() {
            let u = distances
                .iter()
                .filter(|(pos, _)| queue.contains_key(pos))
                .min_by(|(_, dist1), (_, dist2)| dist1.cmp(dist2))
                .map(|(pos, _)| pos)
                .unwrap()
                .clone();

            let palace = queue.remove(&u).unwrap();
            let u_distance = distances.get(&u).unwrap().clone();

            if u_distance == usize::MAX {
                continue;
            }

            palace
                .pos
                .nearest()
                .iter()
                .filter_map(|pos| self.palaces.get(pos))
                .filter(|&it| can_go_to(palace, it))
                .filter(|it| queue.contains_key(&it.pos))
                .for_each(|palace| {
                    let v = palace.pos.clone();
                    let alt = u_distance + 1;

                    if &alt < distances.get(&v).unwrap() {
                        distances.insert(v, alt);
                    }
                });
        }

        self.palaces
            .values()
            .filter(|it| is_end(it))
            .map(|it| distances.get(&it.pos).unwrap())
            .min()
            .unwrap()
            .clone()
    }
}

pub fn run() {
    let elevations: Elevations = read_input(12).into();
    println!(
        "Part1: {}",
        elevations.run(
            |it| matches!(it.type_, PalaceType::S),
            |it| matches!(it.type_, PalaceType::E),
            |palace_from, palace_to| palace_from.can_go_to(palace_to)
        )
    );
    println!(
        "Part2: {}",
        elevations.run(
            |it| matches!(it.type_, PalaceType::E),
            |it| it.height == 0,
            |palace_from, palace_to| palace_to.can_go_to(palace_from)
        )
    );
}
