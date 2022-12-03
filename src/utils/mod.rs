use std::fs::File;
use std::io::prelude::*;

pub fn read_input(day: u32) -> String {
    let mut file = File::open(format!("./src/day{:0>2}/input", day)).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data
}

pub fn lines(input: String) -> Vec<String> {
    input
        .split("\n")
        .filter(|it| !it.is_empty())
        .map(|it| it.to_owned())
        .collect()
}
