use utils::read_input;

pub fn run() {
    part1(read_input!());
    part2(read_input!());
}

fn part1(input: String) {
    let resources: Vec<u32> = get_resources(input);
    println!("Part1: {:?}", resources.iter().max().unwrap());
}

fn part2(input: String) {
    let resources: Vec<u32> = get_resources(input);
    let top3: u32 = resources.iter().take(3).sum();

    println!("Part2: {:?}", top3);
}

fn get_resources(input: String) -> Vec<u32> {
    let mut resources: Vec<u32> = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&inner_input| {
            inner_input
                .split('\n')
                .filter(|it| !it.trim().is_empty())
                .map(|it| it.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    resources.sort_by(|a, b| b.cmp(a));
    resources
}
