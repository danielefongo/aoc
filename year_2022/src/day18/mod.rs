use std::collections::HashSet;

use utils::{lines, read_input};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Cube(i32, i32, i32);
impl Cube {
    fn is_near(&self, other: &Cube) -> bool {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2) == 1
    }
    fn neighbours(&self) -> HashSet<Cube> {
        vec![
            Cube(self.0 - 1, self.1, self.2),
            Cube(self.0 + 1, self.1, self.2),
            Cube(self.0, self.1 + 1, self.2),
            Cube(self.0, self.1 - 1, self.2),
            Cube(self.0, self.1, self.2 + 1),
            Cube(self.0, self.1, self.2 - 1),
        ]
        .into_iter()
        .collect()
    }
    fn all_less(&self, other: &Cube) -> bool {
        self.0 <= other.0 && self.1 <= other.1 && self.2 <= other.2
    }
}
impl From<String> for Cube {
    fn from(input: String) -> Self {
        let values: Vec<i32> = input.split(",").map(|it| it.parse().unwrap()).collect();
        Self(values[0], values[1], values[2])
    }
}

pub fn run() {
    let lava_cubes: HashSet<Cube> = lines(read_input!())
        .into_iter()
        .map(|it| it.into())
        .collect();

    part1(&lava_cubes);
    part2(&lava_cubes);
}

fn part1(lava_cubes: &HashSet<Cube>) {
    let mut count = lava_cubes.len() * 6;
    lava_cubes.iter().enumerate().for_each(|(idx, cube1)| {
        lava_cubes.iter().skip(idx + 1).for_each(|cube2| {
            if cube1.is_near(&cube2) {
                count -= 2;
            }
        })
    });

    println!("Part1: {}", count);
}

fn part2(lava_cubes: &HashSet<Cube>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    lava_cubes.iter().for_each(|c| {
        max_x = max_x.max(c.0);
        max_y = max_y.max(c.1);
        max_z = max_z.max(c.2);
    });

    let start = Cube(0, 0, 0);
    let min = Cube(-1, -1, -1);
    let max = Cube(max_x + 1, max_y + 1, max_z + 1);

    let mut air_block_visited: HashSet<Cube> = HashSet::new();
    let mut queue: Vec<Cube> = vec![start];
    let mut count = 0;

    while let Some(air_cube) = queue.pop() {
        for neighbour in air_cube
            .neighbours()
            .iter()
            .filter(|it| it.all_less(&max) && min.all_less(it))
            .filter(|it| !air_block_visited.contains(it))
            .collect::<Vec<_>>()
        {
            if lava_cubes.contains(&neighbour) {
                count += 1;
            } else if !air_block_visited.contains(neighbour) && !queue.contains(neighbour) {
                queue.push(neighbour.clone())
            }
        }

        air_block_visited.insert(air_cube);
    }

    println!("Part2: {}", count);
}
