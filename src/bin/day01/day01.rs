use advent2020;

fn main() {
    let input = advent2020::read_input("input01");

    let numbers = parse(input);

    println!("Part 1 - {}\n", part1(&numbers));
    println!("Part 2 - {}\n", part2(&numbers));
}

fn parse(input: String) -> Vec<i32> {
    return input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
}

fn part1(numbers: &Vec<i32>) -> i32 {
    find_two_sum_product(numbers, 2020).expect("no two numbers add up to 2020")
}

fn find_two_sum_product(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if numbers[i] + numbers[j] == target {
                return Some(numbers[i] * numbers[j]);
            }
        }
    }

    None
}

fn part2(numbers: &Vec<i32>) -> i32 {
    for i in 0..numbers.len() {
        if numbers[i] < 2020 / 3 {
            match find_two_sum_product(numbers, 2020 - numbers[i]) {
                None => continue,
                Some(n) => return n * numbers[i],
            }
        }
    }

    return 0;
}
