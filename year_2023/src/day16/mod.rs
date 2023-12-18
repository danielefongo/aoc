use std::collections::{HashMap, HashSet};

use utils::{lines, read_input};

#[derive(Debug)]
struct Beam {
    pos: Pos,
    dir: Dir,
}
impl Beam {
    fn new(pos: Pos, dir: Dir) -> Self {
        Self { pos, dir }
    }
    fn to(&self, dir: &Dir) -> Self {
        Beam::new(self.pos.to(dir), dir.clone())
    }
    fn go_on(&self) -> Self {
        self.to(&self.dir)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn to(&self, dir: &Dir) -> Self {
        match dir {
            Dir::Up => Self::new(self.x, self.y - 1),
            Dir::Down => Self::new(self.x, self.y + 1),
            Dir::Left => Self::new(self.x - 1, self.y),
            Dir::Right => Self::new(self.x + 1, self.y),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Point {
    Space,
    VerticalMirror,
    HorizontalMirror,
    SlashMirror,
    BackslashMirror,
}
impl Point {
    fn handle_beam(&self, beam: &Beam) -> Vec<Beam> {
        match (self, &beam.dir) {
            (Point::VerticalMirror, Dir::Left | Dir::Right) => {
                vec![beam.to(&Dir::Up), beam.to(&Dir::Down)]
            }
            (Point::HorizontalMirror, Dir::Up | Dir::Down) => {
                vec![beam.to(&Dir::Left), beam.to(&Dir::Right)]
            }
            (Point::SlashMirror, Dir::Up) => vec![beam.to(&Dir::Right)],
            (Point::SlashMirror, Dir::Down) => vec![beam.to(&Dir::Left)],
            (Point::SlashMirror, Dir::Left) => vec![beam.to(&Dir::Down)],
            (Point::SlashMirror, Dir::Right) => vec![beam.to(&Dir::Up)],
            (Point::BackslashMirror, Dir::Up) => vec![beam.to(&Dir::Left)],
            (Point::BackslashMirror, Dir::Down) => vec![beam.to(&Dir::Right)],
            (Point::BackslashMirror, Dir::Left) => vec![beam.to(&Dir::Up)],
            (Point::BackslashMirror, Dir::Right) => vec![beam.to(&Dir::Down)],
            _ => vec![beam.go_on()],
        }
    }
}
impl From<char> for Point {
    fn from(char: char) -> Self {
        match char {
            '.' => Self::Space,
            '|' => Self::VerticalMirror,
            '-' => Self::HorizontalMirror,
            '/' => Self::SlashMirror,
            '\\' => Self::BackslashMirror,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Debug)]
struct Map {
    points: HashMap<Pos, Point>,
    point_beams: HashMap<Pos, HashSet<Dir>>,
    beams: Vec<Beam>,
    width: i32,
    height: i32,
}
impl Map {
    fn run(&mut self, beam: Beam) -> usize {
        let mut point_beams: HashMap<Pos, HashSet<Dir>> = HashMap::new();
        (*point_beams.entry(beam.pos.clone()).or_default()).insert(Dir::Right);

        self.point_beams = point_beams;
        self.beams = vec![beam];

        while self.next().is_some() {}

        self.point_beams
            .iter()
            .filter(|(_, x)| !x.is_empty())
            .count()
    }
    fn next(&mut self) -> Option<()> {
        let mut new_beams = vec![];
        for beam in self.beams.iter() {
            let generated_beams = self
                .points
                .get(&beam.pos)?
                .handle_beam(beam)
                .into_iter()
                .filter(|beam| {
                    beam.pos.x >= 0
                        && beam.pos.x < self.width
                        && beam.pos.y >= 0
                        && beam.pos.y < self.height
                })
                .collect::<Vec<_>>();

            for new_beam in generated_beams.into_iter() {
                let beams_in_pos = self.point_beams.entry(new_beam.pos.clone()).or_default();
                if beams_in_pos.contains(&new_beam.dir) {
                    continue;
                } else {
                    beams_in_pos.insert(new_beam.dir.clone());
                    new_beams.push(new_beam);
                }
            }
        }
        self.beams = new_beams;

        (!self.beams.is_empty()).then_some(())
    }
}
impl From<Vec<String>> for Map {
    fn from(lines: Vec<String>) -> Self {
        let width = lines[0].len() as i32;
        let height = lines.len() as i32;

        Self {
            points: lines
                .into_iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, char)| (Pos::new(x as i32, y as i32), Point::from(char)))
                        .collect::<Vec<_>>()
                })
                .collect(),
            point_beams: Default::default(),
            beams: Default::default(),
            width,
            height,
        }
    }
}

pub fn run() {
    let mut map = Map::from(lines(read_input!()));

    println!(
        "Part1: {:?}",
        map.run(Beam::new(Pos::new(0, 0), Dir::Right))
    );

    let mut max = 0;
    for x in 0..map.width {
        max = max.max(map.run(Beam::new(Pos::new(x, 0), Dir::Down)));
        max = max.max(map.run(Beam::new(Pos::new(x, map.height - 1), Dir::Up)));
    }
    for y in 0..map.width {
        max = max.max(map.run(Beam::new(Pos::new(0, y), Dir::Right)));
        max = max.max(map.run(Beam::new(Pos::new(map.width - 1, y), Dir::Left)));
    }
    println!("Part2: {}", max);
}
