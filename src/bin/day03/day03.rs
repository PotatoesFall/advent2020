use std::fs;

fn main() {
    let input = fs::read_to_string("input/input03").unwrap();

    let trees: Vec<Vec<bool>> = input
        .split("\n")
        .map(|line| line.chars().map(|ch| ch == '#').collect())
        .collect();

    let slope = (3, 1);
    let part_1 = ride_slope(&trees, slope);
    println!("Part 1 - {}", part_1);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut part_2: i64 = 1;
    for slope in slopes {
        part_2 *= ride_slope(&trees, slope);
    }

    println!("Part 2 - {}", part_2)
}

fn ride_slope(trees: &Vec<Vec<bool>>, slope: (usize, usize)) -> i64 {
    let rows = trees.len();
    let cols = trees[0].len();

    let mut x = 0;
    let mut y = 0;

    let mut count = 0;
    while y < rows {
        if trees[y][x] {
            count += 1;
        }

        x = (x + slope.0) % cols;
        y = y + slope.1;
    }

    count
}
