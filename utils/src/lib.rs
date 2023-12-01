use regex::Regex;

#[macro_export]
macro_rules! read_input {
    () => {
        include_str!("./input").to_string()
    };
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
