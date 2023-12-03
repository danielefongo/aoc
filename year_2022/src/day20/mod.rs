use utils::{lines, read_input};

pub fn run() {
    let input1 = lines(read_input!())
        .into_iter()
        .map(|it| it.parse::<i128>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();

    let input2 = lines(read_input!())
        .into_iter()
        .map(|it| it.parse::<i128>().unwrap() * 811589153)
        // .map(|it| it.parse::<i128>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();

    println!("Part1: {}", part1(input1, 1));
    println!("Part2: {}", part1(input2, 10));
}

fn part1(input: Vec<(usize, i128)>, iterations: usize) -> i128 {
    let mut input = input;
    for _iteration in 0..iterations {
        for idx in 0..input.len() {
            do_swap(&mut input, idx);
        }
    }

    let zero_idx = find(&input, |(_, movement)| movement == &0).0;

    let result_new = [
        input.get((zero_idx + 1000) % input.len()).unwrap().1,
        input.get((zero_idx + 2000) % input.len()).unwrap().1,
        input.get((zero_idx + 3000) % input.len()).unwrap().1,
    ];

    result_new.into_iter().sum::<i128>()
}

fn do_swap(input: &mut Vec<(usize, i128)>, reference: usize) {
    let (old_pos, (_, movement)) = find(input, |(idx, _)| idx == &reference);
    let old_pos = old_pos as i128;

    let len = input.len() as i128;
    let reduced_len = len - 1;

    let delta = movement.signum();
    let steps = movement.abs() % reduced_len;

    (0..steps.abs()).for_each(|step| {
        let my_pos = (old_pos + step * delta).rem_euclid(len);
        let his_pos = (old_pos + step * delta + delta).rem_euclid(len);

        input.swap(my_pos as usize, his_pos as usize);
    });

    let shift = if old_pos.rem_euclid(reduced_len) == 0 {
        0
    } else {
        (movement.abs() - steps + 1) % len
    };

    match shift {
        s if s < 0 => input.rotate_left(s.unsigned_abs() as usize),
        s if s > 0 => input.rotate_right(s.unsigned_abs() as usize),
        _ => {}
    }
}

fn find(vec: &[(usize, i128)], filter: impl Fn(&(usize, i128)) -> bool) -> (usize, (usize, i128)) {
    vec.iter()
        .cloned()
        .enumerate()
        .find(|(_, data)| filter(data))
        .unwrap()
}
