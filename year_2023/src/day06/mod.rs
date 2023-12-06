use utils::{extract, lines, read_input, replace};

pub fn run() {
    let lines = lines(replace(&read_input!(), "\\w+:", ""))
        .into_iter()
        .map(|it| {
            extract(&it, "\\d+")
                .into_iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let runs = lines[0]
        .clone()
        .into_iter()
        .zip(lines[1].clone())
        .map(|(time, distance)| {
            (1..time)
                .map(|it| it * (time - it))
                .filter(|it| it > &distance)
                .count()
        })
        .product::<usize>();

    println!("Part1: {:?}", runs)
}
