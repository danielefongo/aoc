use crate::utils::read_input;

pub fn run() {
    let matrix = create_matrix(read_input(8));
    let (width, height) = (matrix.len(), matrix.first().unwrap().len());

    let mut count = width * 2 + height * 2 - 4;

    matrix
        .iter()
        .enumerate()
        .skip(1)
        .take(matrix.len() - 2)
        .for_each(|(y, vec)| {
            vec.iter()
                .enumerate()
                .skip(1)
                .take(vec.len() - 2)
                .for_each(|(x, &height)| {
                    if visible_right(height, (x, y), &matrix)
                        || visible_left(height, (x, y), &matrix)
                        || visible_top(height, (x, y), &matrix)
                        || visible_bottom(height, (x, y), &matrix)
                    {
                        count += 1;
                    }
                })
        });

    println!("Part1: {}", count);
}

fn visible_right(height: usize, (x, y): (usize, usize), matrix: &Vec<Vec<usize>>) -> bool {
    matrix[y].iter().skip(x + 1).max().unwrap() < &height
}

fn visible_left(height: usize, (x, y): (usize, usize), matrix: &Vec<Vec<usize>>) -> bool {
    matrix[y].iter().take(x).max().unwrap() < &height
}

fn visible_bottom(height: usize, (x, y): (usize, usize), matrix: &Vec<Vec<usize>>) -> bool {
    matrix.iter().skip(y + 1).map(|it| it[x]).max().unwrap() < height
}

fn visible_top(height: usize, (x, y): (usize, usize), matrix: &Vec<Vec<usize>>) -> bool {
    matrix.iter().take(y).map(|it| it[x]).max().unwrap() < height
}

fn create_matrix(data: String) -> Vec<Vec<usize>> {
    data.split("\n")
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}
