use std::collections::{BTreeSet, HashMap};

use utils::read_input;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
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

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum PalaceType {
    S,
    E,
    N,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Palace {
    type_: PalaceType,
    pos: Pos,
    height: u8,
}
impl Palace {
    fn new(type_: PalaceType, pos: Pos, height: u8) -> Self {
        Self { type_, pos, height }
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
                            let palace = match palace {
                                'S' => Palace::new(PalaceType::S, pos, 0),
                                'E' => Palace::new(PalaceType::E, pos, 25),
                                it => Palace::new(PalaceType::N, pos, (it as u8) - ('a' as u8)),
                            };
                            (pos, palace)
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
        &mut self,
        is_start: fn(&Palace) -> bool,
        is_end: fn(&Palace) -> bool,
        can_go_to: fn(&Palace, &Palace) -> bool,
    ) -> usize {
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        let mut queue: BTreeSet<(usize, &Palace)> = BTreeSet::new();

        for palace in self.palaces.values() {
            let distance = if is_start(palace) { 0 } else { usize::MAX };

            queue.insert((distance, palace));
            distances.insert(palace.pos, distance);
        }

        while !queue.is_empty() {
            let (u_distance, palace) = queue.iter().next().unwrap().clone();
            let u = palace.pos;

            queue.remove(&(u_distance, palace));

            if u_distance == usize::MAX {
                continue;
            }

            let neighbours = u
                .nearest()
                .iter()
                .filter_map(|pos| self.palaces.get(pos))
                .filter(|&it| can_go_to(palace, it))
                .collect::<Vec<&Palace>>();

            for palace in neighbours {
                let v = palace.pos.clone();
                let v_distance = distances.get(&v).unwrap();
                let alt = u_distance + 1;

                if &alt < v_distance {
                    queue.remove(&(*v_distance, palace));
                    queue.insert((alt, palace));
                    distances.insert(v, alt);
                }
            }
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
    let mut elevations: Elevations = read_input!().into();
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
