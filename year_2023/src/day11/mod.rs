use utils::{lines, read_input};

#[derive(Debug)]
struct Pos {
    x: i64,
    y: i64,
}
impl Pos {
    fn distance(&self, other: &Pos) -> i64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64
    }
}

#[derive(Debug)]
struct Galaxy {
    pos: Pos,
}
impl Galaxy {
    fn distance(&self, other: &Galaxy) -> i64 {
        self.pos.distance(&other.pos)
    }
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Galaxy>,
}
impl Universe {
    fn expand(&mut self, age: i64) {
        let max_x = self.galaxies.iter().map(|it| it.pos.x).max().unwrap();
        let max_y = self.galaxies.iter().map(|it| it.pos.y).max().unwrap();

        (0..max_x).rev().for_each(|x| {
            if !self.galaxies.iter().any(|it| it.pos.x == x) {
                self.galaxies
                    .iter_mut()
                    .filter(|it| it.pos.x > x)
                    .for_each(|it| {
                        it.pos.x += age - 1;
                    })
            }
        });
        (0..max_y).rev().for_each(|y| {
            if !self.galaxies.iter().any(|it| it.pos.y == y) {
                self.galaxies
                    .iter_mut()
                    .filter(|it| it.pos.y > y)
                    .for_each(|it| {
                        it.pos.y += age - 1;
                    })
            }
        });
    }
}
impl From<Vec<String>> for Universe {
    fn from(lines: Vec<String>) -> Self {
        Self {
            galaxies: lines
                .into_iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| c == &'#')
                        .map(|(x, _)| Galaxy {
                            pos: Pos {
                                x: x as i64,
                                y: y as i64,
                            },
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
}

pub fn run() {
    println!("Part1: {:?}", runner(2));
    println!("Part2: {:?}", runner(1000000));
}

fn runner(age: i64) -> i64 {
    let mut uni = Universe::from(lines(read_input!()));
    uni.expand(age);
    uni.galaxies
        .iter()
        .enumerate()
        .flat_map(|(idx, g1)| {
            uni.galaxies
                .iter()
                .skip(idx)
                .map(|g2| g1.distance(g2))
                .collect::<Vec<_>>()
        })
        .sum::<i64>()
}
