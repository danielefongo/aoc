use utils::read_input;

type Pos = (usize, usize);
type Trees = Vec<usize>;
type Matrix = Vec<Trees>;

pub fn run() {
    let matrix = create_matrix(read_input!());

    println!("Part1: {}", walk(matrix.len() * 4 - 4, &matrix, updater1));
    println!("Part2: {}", walk(None, &matrix, updater2).unwrap());
}

fn walk<T>(initial: T, matrix: &Matrix, updater: fn(&T, usize, Pos, &Matrix) -> T) -> T {
    let (width, height) = (matrix.len(), matrix.first().unwrap().len());

    let mut actual = initial;

    matrix
        .iter()
        .enumerate()
        .skip(1)
        .take(height - 2)
        .for_each(|(y, vec)| {
            vec.iter()
                .enumerate()
                .skip(1)
                .take(width - 2)
                .for_each(|(x, &height)| {
                    actual = updater(&actual, height, (x, y), &matrix);
                })
        });

    actual
}

fn updater1(count: &usize, height: usize, pos: Pos, matrix: &Matrix) -> usize {
    if tallest(height, right_of(pos, &matrix))
        || tallest(height, left_of(pos, &matrix))
        || tallest(height, top_of(pos, &matrix))
        || tallest(height, bottom_of(pos, &matrix))
    {
        count + 1
    } else {
        count.clone()
    }
}

fn updater2(best: &Option<usize>, height: usize, pos: Pos, matrix: &Matrix) -> Option<usize> {
    let score = count_of_visible_trees(height, right_of(pos, &matrix))
        * count_of_visible_trees(height, left_of(pos, &matrix))
        * count_of_visible_trees(height, top_of(pos, &matrix))
        * count_of_visible_trees(height, bottom_of(pos, &matrix));

    best.map(|it| it.max(score)).or_else(|| Some(score))
}

fn right_of((x, y): (usize, usize), matrix: &Matrix) -> Trees {
    matrix[y].iter().skip(x + 1).cloned().collect()
}

fn left_of((x, y): (usize, usize), matrix: &Matrix) -> Trees {
    matrix[y].iter().take(x).rev().cloned().collect()
}

fn bottom_of((x, y): (usize, usize), matrix: &Matrix) -> Trees {
    matrix.iter().skip(y + 1).map(|it| it[x]).collect()
}

fn top_of((x, y): (usize, usize), matrix: &Matrix) -> Trees {
    matrix.iter().take(y).rev().map(|it| it[x]).collect()
}

fn tallest(height: usize, other_trees: Trees) -> bool {
    other_trees.iter().max().unwrap() < &height
}

fn count_of_visible_trees(height: usize, other_trees: Trees) -> usize {
    let count = other_trees.iter().take_while(|&it| it < &height).count();
    if count > 0 && count < other_trees.len() && other_trees[count] >= height {
        count + 1
    } else {
        count
    }
}

fn create_matrix(data: String) -> Matrix {
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
