use utils::{extract, lines, read_input, replace};

pub fn run() {
    let input = replace(&read_input!(), "\\w+:", "");

    println!("Part1: {:?}", runner(input.clone()));
    println!("Part2: {:?}", runner(input.replace(' ', "")));
}

fn runner(input: String) -> usize {
    let lines = lines(input)
        .into_iter()
        .map(|it| {
            extract(&it, "\\d+")
                .into_iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    lines[0]
        .clone()
        .into_iter()
        .zip(lines[1].clone())
        .map(|(time, distance)| {
            (1..time)
                .map(|it| it * (time - it))
                .filter(|it| it > &distance)
                .count()
        })
        .product::<usize>()
}
