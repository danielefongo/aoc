use crate::utils::{lines, read_input};

#[derive(Debug)]
struct Cube(i32, i32, i32);
impl Cube {
    fn is_near(&self, other: &Cube) -> bool {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2) == 1
    }
}
impl From<String> for Cube {
    fn from(input: String) -> Self {
        let values: Vec<i32> = input.split(",").map(|it| it.parse().unwrap()).collect();
        Self(values[0], values[1], values[2])
    }
}

pub fn run() {
    let cubes: Vec<Cube> = lines(read_input(18))
        .into_iter()
        .map(|it| it.into())
        .collect();

    let mut count = cubes.len() * 6;
    cubes.iter().enumerate().for_each(|(idx, cube1)| {
        cubes.iter().skip(idx + 1).for_each(|cube2| {
            if cube1.is_near(&cube2) {
                count -= 2;
            }
        })
    });

    println!("Part1: {}", count);
}
