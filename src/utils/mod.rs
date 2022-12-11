use regex::Regex;
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

pub fn matches(data: &str, regex: &str) -> bool {
    let regex = Regex::new(regex).unwrap();
    regex.is_match(&data)
}

pub fn extract(data: &str, regex: &str) -> Vec<String> {
    Regex::new(regex)
        .unwrap()
        .find_iter(&data)
        .map(|digits| digits.as_str().to_owned())
        .collect()
}

pub fn extract_one(data: &str, regex: &str) -> String {
    extract(data, regex).first().unwrap().clone()
}
