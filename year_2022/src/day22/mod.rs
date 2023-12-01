use std::collections::HashMap;

use utils::{extract, lines, read_input};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(i32, i32);
impl Pos {
    fn move_to(&self, dir: &Dir, steps: i32) -> Pos {
        match dir {
            Dir::Up => Pos(self.0, self.1 - steps),
            Dir::Down => Pos(self.0, self.1 + steps),
            Dir::Left => Pos(self.0 - steps, self.1),
            Dir::Right => Pos(self.0 + steps, self.1),
        }
    }
    fn all_pos(&self, dir: &Dir, steps: i32) -> Vec<Pos> {
        (0..(steps as usize))
            .map(|s| self.move_to(dir, s as i32))
            .collect()
    }
}

#[derive(Debug)]
struct Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}
impl Dir {
    fn rotate_left(&self) -> Dir {
        match self {
            Dir::Up => Self::Left,
            Dir::Down => Self::Right,
            Dir::Right => Self::Up,
            Dir::Left => Self::Down,
        }
    }
    fn rotate_right(&self) -> Dir {
        match self {
            Dir::Up => Self::Right,
            Dir::Down => Self::Left,
            Dir::Right => Self::Down,
            Dir::Left => Self::Up,
        }
    }
    fn opposite(&self) -> Dir {
        self.rotate_right().rotate_right()
    }
}

#[derive(Debug)]
enum Step {
    Straight(i32),
    Left,
    Right,
}
impl From<String> for Step {
    fn from(input: String) -> Self {
        match &input[..] {
            "R" => Step::Right,
            "L" => Step::Left,
            value => Step::Straight(value.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
struct Steps {
    steps: Vec<Step>,
}
impl From<String> for Steps {
    fn from(input: String) -> Self {
        let steps = extract(&input, "(\\d+|R|L)")
            .into_iter()
            .map(|it| it.into())
            .collect::<Vec<_>>();
        Self { steps }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Kind {
    Free,
    Wall,
}
impl TryFrom<char> for Kind {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Kind::Free),
            '#' => Ok(Kind::Wall),
            _ => Err(Error),
        }
    }
}

#[derive(Debug)]
struct Map {
    fields: HashMap<Pos, Kind>,
    actual_pos: Pos,
    max_x: i32,
    max_y: i32,
    dir: Dir,
}
impl Map {
    fn do_step(&mut self, step: &Step, pos_calculator: &Box<dyn PosCalculator>) {
        match step {
            Step::Straight(steps) => {
                let mut count = 0;
                while &count < steps {
                    let (new_pos, new_dir) = pos_calculator.next_pos(self.actual_pos, self.dir);

                    if let Some(Kind::Wall) = self.fields.get(&new_pos) {
                        break;
                    }

                    count += 1;
                    self.actual_pos = new_pos;
                    self.dir = new_dir;
                }
            }
            Step::Left => self.dir = self.dir.rotate_left(),
            Step::Right => self.dir = self.dir.rotate_right(),
        }
    }
    fn value(&self) -> i32 {
        let dir_value = match self.dir {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        };
        let Pos(x, y) = self.actual_pos;

        1000 * (y + 1) + 4 * (x + 1) + dir_value
    }
}
impl From<Vec<String>> for Map {
    fn from(input: Vec<String>) -> Self {
        let fields: HashMap<Pos, Kind> = input
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .into_iter()
                    .enumerate()
                    .flat_map(|(x, char)| Kind::try_from(char).map(|kind| (x, kind)))
                    .map(|(x, kind)| {
                        // dbg!(x, y, &kind);
                        (Pos(x as i32, y as i32), kind)
                    })
                    .collect::<Vec<(Pos, Kind)>>()
            })
            .collect();
        let actual_pos = fields
            .iter()
            .filter(|(_, it)| *it == &Kind::Free)
            .map(|it| it.0)
            .min_by(|Pos(x1, y1), Pos(x2, y2)| (y1, x1).cmp(&(y2, x2)))
            .unwrap()
            .clone();

        let max_x = fields.keys().map(|it| it.0).max().unwrap();
        let max_y = fields.keys().map(|it| it.1).max().unwrap();
        Self {
            fields,
            actual_pos,
            max_x,
            max_y,
            dir: Dir::Right,
        }
    }
}

type RelativeFace = (Dir, Face);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Face {
    Top,
    Left,
    Right,
    Front,
    Back,
    Bot,
}
impl Face {
    fn other_relative_faces(&self, (relative_dir, face): &RelativeFace) -> Vec<RelativeFace> {
        let dirs = vec![Dir::Up, Dir::Left, Dir::Down, Dir::Right];
        let mut faces = match self {
            Face::Top => vec![Face::Back, Face::Left, Face::Front, Face::Right],
            Face::Left => vec![Face::Top, Face::Back, Face::Bot, Face::Front],
            Face::Right => vec![Face::Top, Face::Front, Face::Bot, Face::Back],
            Face::Front => vec![Face::Top, Face::Left, Face::Bot, Face::Right],
            Face::Back => vec![Face::Top, Face::Right, Face::Bot, Face::Left],
            Face::Bot => vec![Face::Front, Face::Left, Face::Back, Face::Right],
        };

        let index_of_face = faces.iter().position(|it| it == face).unwrap() as i32;
        let index_of_dir = dirs.iter().position(|it| it == relative_dir).unwrap() as i32;
        let shift = index_of_dir - index_of_face;
        if shift < 0 {
            faces.rotate_left(-shift as usize)
        } else {
            faces.rotate_right(shift as usize)
        }

        dirs.into_iter().zip(faces).collect()
    }
    fn go_to(&self, relative_face: &RelativeFace, absolute_dir: &Dir) -> Option<Face> {
        self.other_relative_faces(relative_face)
            .iter()
            .find(|(dir, _)| dir == absolute_dir)
            .map(|it| it.1)
    }
    fn find_by_dir(&self, relative_face: &RelativeFace, other_dir: &Dir) -> RelativeFace {
        self.other_relative_faces(relative_face)
            .into_iter()
            .find(|(d, _)| d == other_dir)
            .unwrap()
    }
    fn find_by_face(&self, relative_face: &RelativeFace, other_face: &Face) -> RelativeFace {
        self.other_relative_faces(relative_face)
            .into_iter()
            .find(|(_, f)| f == other_face)
            .unwrap()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Facet {
    face: Face,
    start: Pos,
    end: Pos,
    size: i32,
}
impl Facet {
    fn new(face: Face, start: Pos, end: Pos) -> Self {
        let size = end.0 - start.0 + 1;
        Self {
            face,
            start,
            end,
            size,
        }
    }
    fn edge(&self, dir: Dir) -> Vec<Pos> {
        match dir {
            Dir::Up => self.start.all_pos(&Dir::Right, self.size),
            Dir::Down => self
                .end
                .all_pos(&Dir::Left, self.size)
                .into_iter()
                .rev()
                .collect(),
            Dir::Right => self.end.all_pos(&Dir::Up, self.size),
            Dir::Left => self
                .start
                .all_pos(&Dir::Down, self.size)
                .into_iter()
                .rev()
                .collect(),
        }
    }
}

trait PosCalculator {
    fn next_pos(&self, from: Pos, dir: Dir) -> (Pos, Dir);
}

struct Roll {
    fields: HashMap<Pos, Kind>,
    max_x: i32,
    max_y: i32,
}
impl Roll {
    fn roll(&self, from: &Pos, dir: &Dir) -> Pos {
        match dir {
            Dir::Up => Pos(from.0, self.max_y + 1),
            Dir::Down => Pos(from.0, -1),
            Dir::Left => Pos(self.max_x + 1, from.1),
            Dir::Right => Pos(-1, from.1),
        }
    }
}
impl From<&Map> for Roll {
    fn from(map: &Map) -> Self {
        Roll {
            max_x: map.max_x,
            max_y: map.max_y,
            fields: map.fields.clone(),
        }
    }
}
impl PosCalculator for Roll {
    fn next_pos(&self, from: Pos, dir: Dir) -> (Pos, Dir) {
        let next_pos = from.move_to(&dir, 1);
        if self.fields.get(&next_pos).is_some() {
            (next_pos, dir)
        } else {
            let mut rolled = self.roll(&next_pos, &dir);
            while self.fields.get(&rolled).is_none() {
                rolled = rolled.move_to(&dir, 1);
            }
            (rolled.clone(), dir)
        }
    }
}

struct Cube {
    edges: HashMap<Face, (Dir, Face)>,
    facets: Vec<Facet>,
}
impl PosCalculator for Cube {
    fn next_pos(&self, from: Pos, dir: Dir) -> (Pos, Dir) {
        let moved_pos = from.move_to(&dir, 1);

        if let Some(_) = self.find_by_pos(moved_pos) {
            (moved_pos, dir)
        } else {
            let from_facet = self.find_by_pos(from).unwrap();
            let from_face = from_facet.face;

            let (from_to_dir, to_face) = self.find_relative_from_dir(from_face, dir);
            let (to_from_dir, _) = self.find_relative_from_face(to_face, from_face);

            let same_edge_direction = match (from_to_dir, to_from_dir) {
                (Dir::Up, Dir::Right) => true,
                (Dir::Down, Dir::Left) => true,
                (Dir::Right, Dir::Up) => true,
                (Dir::Left, Dir::Down) => true,
                (x, y) if x == y.opposite() => true,
                _ => false,
            };

            let to_facet = self.find_by_face(to_face).unwrap();

            let first_edge = from_facet.edge(from_to_dir);
            let second_edge = to_facet.edge(to_from_dir);
            let actual_edge_idx = first_edge.iter().position(|it| it == &from).unwrap();

            let direction = to_from_dir.opposite();

            let pos = if same_edge_direction {
                second_edge.get(actual_edge_idx).unwrap().clone()
            } else {
                second_edge
                    .get((from_facet.size - 1 - actual_edge_idx as i32) as usize)
                    .unwrap()
                    .clone()
            };

            (pos, direction)
        }
    }
}
impl From<&Map> for Cube {
    fn from(map: &Map) -> Self {
        let mut cube = Cube {
            edges: HashMap::new(),
            facets: Vec::new(),
        };

        cube.build(map, map.actual_pos.clone(), Face::Top);
        cube
    }
}
impl Cube {
    fn build(&mut self, map: &Map, pos: Pos, face: Face) {
        let steps = (map.max_x + 1).min(map.max_y + 1) / 3;
        let size = steps - 1;

        [Dir::Up, Dir::Right, Dir::Down, Dir::Left]
            .into_iter()
            .for_each(|dir| {
                let facet_start = pos;
                let facet_end = pos.move_to(&Dir::Right, size).move_to(&Dir::Down, size);
                let new_facet_start = pos.move_to(&dir, steps);
                let new_facet_end = new_facet_start
                    .move_to(&Dir::Right, size)
                    .move_to(&Dir::Down, size);

                if map.fields.contains_key(&new_facet_start) {
                    let new_face = if matches!(face, Face::Top) {
                        face.go_to(&(Dir::Up, Face::Back), &dir).unwrap()
                    } else {
                        let relative_face = self.edges.get(&face).unwrap();
                        face.go_to(relative_face, &dir).unwrap()
                    };

                    if !self.edges.contains_key(&face) {
                        self.facets.push(Facet::new(face, facet_start, facet_end));
                        self.edges.insert(face, (dir, new_face));
                    }
                    if !self.edges.contains_key(&new_face) {
                        self.facets
                            .push(Facet::new(new_face, new_facet_start, new_facet_end));
                        self.edges.insert(new_face, (dir.opposite(), face));
                        self.build(map, new_facet_start, new_face)
                    }
                }
            });
    }
    fn find_by_pos(&self, pos: Pos) -> Option<&Facet> {
        self.facets.iter().find(|it| {
            it.start.0 <= pos.0 && it.start.1 <= pos.1 && it.end.0 >= pos.0 && it.end.1 >= pos.1
        })
    }
    fn find_by_face(&self, face: Face) -> Option<&Facet> {
        self.facets.iter().find(|it| it.face == face)
    }
    fn find_relative_from_dir(&self, from: Face, absolute_dir: Dir) -> RelativeFace {
        let from_relative_face = self.edges.get(&from).unwrap();
        from.find_by_dir(from_relative_face, &absolute_dir)
    }
    fn find_relative_from_face(&self, from: Face, other_face: Face) -> RelativeFace {
        let from_relative_face = self.edges.get(&from).unwrap();
        from.find_by_face(from_relative_face, &other_face)
    }
}

pub fn run() {
    let mut lines = lines(read_input!());
    let steps = Steps::from(lines.last().unwrap().clone());

    lines.pop();
    let mut first_map = Map::from(lines.clone());
    let roll: Box<dyn PosCalculator> = Box::new(Roll::from(&first_map));

    let mut second_map = Map::from(lines);
    let cube: Box<dyn PosCalculator> = Box::new(Cube::from(&second_map));

    let mut step_iter = steps.steps.iter();
    while let Some(step) = step_iter.next() {
        first_map.do_step(step, &roll);
        second_map.do_step(step, &cube);
    }

    println!("Part1: {}", first_map.value());
    println!("Part2: {}", second_map.value());
}

#[cfg(test)]
mod tests {
    use super::{Dir, Face};

    #[test]
    fn face() {
        assert_eq!(
            Face::Right.other_relative_faces(&(Dir::Left, Face::Top)),
            vec![
                (Dir::Up, Face::Back),
                (Dir::Left, Face::Top),
                (Dir::Down, Face::Front),
                (Dir::Right, Face::Bot),
            ]
        );
        assert_eq!(
            Face::Front.other_relative_faces(&(Dir::Up, Face::Top)),
            vec![
                (Dir::Up, Face::Top),
                (Dir::Left, Face::Left),
                (Dir::Down, Face::Bot),
                (Dir::Right, Face::Right),
            ]
        );
        assert_eq!(
            Face::Top.other_relative_faces(&(Dir::Right, Face::Right)),
            vec![
                (Dir::Up, Face::Back),
                (Dir::Left, Face::Left),
                (Dir::Down, Face::Front),
                (Dir::Right, Face::Right),
            ]
        );
        assert_eq!(
            Face::Left.other_relative_faces(&(Dir::Right, Face::Bot)),
            vec![
                (Dir::Up, Face::Front),
                (Dir::Left, Face::Top),
                (Dir::Down, Face::Back),
                (Dir::Right, Face::Bot),
            ]
        );
        assert_eq!(
            Face::Back.other_relative_faces(&(Dir::Up, Face::Left)),
            vec![
                (Dir::Up, Face::Left),
                (Dir::Left, Face::Top),
                (Dir::Down, Face::Right),
                (Dir::Right, Face::Bot),
            ]
        );
    }

    mod cube {
        use std::collections::HashMap;

        use crate::day22::{Cube, Dir, Face, Facet, Map, Pos, PosCalculator};

        #[test]
        fn build_from_map() {
            let map = Map::from(vec![
                "   ......".to_string(),
                "   ......".to_string(),
                "   ......".to_string(),
                "   ...   ".to_string(),
                "   ...   ".to_string(),
                "   ...   ".to_string(),
                "......   ".to_string(),
                "......   ".to_string(),
                "......   ".to_string(),
                "...      ".to_string(),
                "...      ".to_string(),
                "...      ".to_string(),
            ]);
            let cube = Cube::from(&map);

            assert_eq!(
                cube.edges,
                vec![
                    (Face::Top, (Dir::Right, Face::Right)),
                    (Face::Right, (Dir::Left, Face::Top)),
                    (Face::Front, (Dir::Up, Face::Top)),
                    (Face::Left, (Dir::Right, Face::Bot)),
                    (Face::Bot, (Dir::Up, Face::Front)),
                    (Face::Back, (Dir::Up, Face::Left)),
                ]
                .into_iter()
                .collect::<HashMap<_, _>>()
            );

            assert_eq!(
                cube.facets,
                vec![
                    Facet::new(Face::Top, Pos(3, 0), Pos(5, 2)),
                    Facet::new(Face::Right, Pos(6, 0), Pos(8, 2)),
                    Facet::new(Face::Front, Pos(3, 3), Pos(5, 5)),
                    Facet::new(Face::Bot, Pos(3, 6), Pos(5, 8)),
                    Facet::new(Face::Left, Pos(0, 6), Pos(2, 8)),
                    Facet::new(Face::Back, Pos(0, 9), Pos(2, 11)),
                ]
            );
        }

        #[test]
        fn next_pos() {
            let map = Map::from(vec![
                "   ......".to_string(),
                "   ......".to_string(),
                "   ......".to_string(),
                "   ...   ".to_string(),
                "   ...   ".to_string(),
                "   ...   ".to_string(),
                "......   ".to_string(),
                "......   ".to_string(),
                "......   ".to_string(),
                "...      ".to_string(),
                "...      ".to_string(),
                "...      ".to_string(),
            ]);
            let cube = Cube::from(&map);

            assert_eq!(
                cube.next_pos(Pos(3, 0), Dir::Right),
                (Pos(4, 0), Dir::Right)
            );
            assert_eq!(cube.next_pos(Pos(5, 3), Dir::Right), (Pos(6, 2), Dir::Up));
            assert_eq!(cube.next_pos(Pos(5, 6), Dir::Right), (Pos(8, 2), Dir::Left));
            assert_eq!(cube.next_pos(Pos(0, 8), Dir::Left), (Pos(3, 0), Dir::Right));
            assert_eq!(cube.next_pos(Pos(0, 11), Dir::Down), (Pos(6, 0), Dir::Down));
            assert_eq!(cube.next_pos(Pos(2, 11), Dir::Right), (Pos(5, 8), Dir::Up));
            assert_eq!(cube.next_pos(Pos(3, 0), Dir::Up), (Pos(0, 9), Dir::Right));
        }
    }
}
